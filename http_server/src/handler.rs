use http::{
    http_request::{self, HttpRequest},
    http_response::HttpResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(file: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let pubblic_path = std::env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", pubblic_path, file);

        let content = std::fs::read_to_string(full_path);
        content.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub struct StaticPage;
impl Handler for StaticPage {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http_request::Resource::Path(path) = &req.resource;

        let route: Vec<&str> = path.split("/").collect();
        match route[1] {
            // match first route level
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "test" => HttpResponse::new("200", None, Self::load_file("test.html")),
            other_path => match Self::load_file(other_path) {
                Some(content) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if other_path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if other_path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(content))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

pub struct PageNotFound;
impl Handler for PageNotFound {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

pub struct WebService;
impl WebService {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = std::env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_content = std::fs::read_to_string(full_path);
        let orders = serde_json::from_str(json_content.unwrap().as_str()).unwrap();
        orders
    }
}
impl Handler for WebService {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http_request::Resource::Path(path) = &req.resource;

        let route: Vec<&str> = path.split("/").collect();
        match route[2] {
            // match second route level
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}

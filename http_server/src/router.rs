use super::handler::{Handler, PageNotFound, StaticPage, WebService};
use http::{http_request, http_request::HttpRequest, http_response::HttpResponse};
use std::io::prelude::*;

pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        match req.method {
            http_request::Method::GET => match &req.resource {
                http_request::Resource::Path(p) => {
                    let route: Vec<&str> = p.split("/").collect();
                    match route[1] {
                        "api" => {
                            let res: HttpResponse = WebService::handle(&req);
                            let _ = res.send_response(stream);
                        }
                        _ => {
                            let res: HttpResponse = StaticPage::handle(&req);
                            let _ = res.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let res: HttpResponse = PageNotFound::handle(&req);
                let _ = res.send_response(stream);
            }
        }
    }
}

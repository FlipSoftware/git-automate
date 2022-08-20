use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    Unitialized,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    V1_1,
    V2_0,
    Unitialized,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Resource {
    Path(String),
}
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<&str> for Method {
    fn from(arg: &str) -> Self {
        match arg {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Unitialized,
        }
    }
}

impl From<&str> for Version {
    fn from(arg: &str) -> Self {
        match arg {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Unitialized,
        }
    }
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Unitialized;
        let mut parsed_version = Version::Unitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // blank
            } else {
                parsed_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: parsed_body.to_string(),
        }
    }
}

fn process_req_line(arg: &str) -> (Method, Resource, Version) {
    let mut words = arg.split_whitespace();
    let method = words.next().unwrap();
    let resource_path = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource_path.into()),
        version.into(),
    )
}
fn process_header_line(arg: &str) -> (String, String) {
    let mut parsed_header = arg.split(':');
    let key = parsed_header.next().unwrap().to_string();
    let value = parsed_header.next().unwrap().to_string();

    (key, value)
}

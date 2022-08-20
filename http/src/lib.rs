pub mod http_request;
pub mod http_response;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::http_request::*;

    #[test]
    fn test_method_into() {
        let method: Method = "GET".into();
        assert_eq!(method, Method::GET);
    }
    #[test]
    fn test_version_int() {
        let version: Version = "HTTP/1.1".into();
        assert_eq!(version, Version::V1_1);
    }
    #[test]
    fn test_read_http() {
        let req = String::from(
            "GET /test HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: xh\r\nAccept: */*\r\n\r\n",
        );
        let mut expected_headers = HashMap::new();
        expected_headers.insert("Host".into(), " localhost".into());
        expected_headers.insert("Accept".into(), " */*".into());
        expected_headers.insert("User-Agent".into(), " xh".into());
        let req: HttpRequest = req.into();
        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/test".into()), req.resource);
        assert_eq!(expected_headers, req.headers);
    }
}

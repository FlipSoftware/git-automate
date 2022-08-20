use std::{collections::HashMap, io::Write};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_msg: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_msg: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_msg(),
            res.headers(),
            res.body().len(),
            res.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code;
        };
        response.headers = match headers {
            Some(_) => headers,
            None => {
                let mut init_header = HashMap::new();
                init_header.insert("Content-Type:", "text/html");
                Some(init_header)
            }
        };
        response.status_msg = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Unreacheable",
        };
        response.body = body;

        response
    }

    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_msg(&self) -> &str {
        self.status_msg
    }
    fn headers(&self) -> String {
        let header_iter = self.headers.clone().unwrap();
        let mut header_text = String::new();

        for (key, value) in header_iter {
            header_text = format!("{key}{value}\r\n");
        }
        header_text
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b,
            None => "",
        }
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), std::io::Error> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_200() {
        let response = HttpResponse::new("200", None, Some("Body message".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_msg: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type:", "text/html");
                Some(h)
            },
            body: Some("Body message".into()),
        };
        assert_eq!(response, response_expected);
    }
    #[test]
    fn test_response_404() {
        let response = HttpResponse::new("404", None, Some("Body message".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_msg: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type:", "text/html");
                Some(h)
            },
            body: Some("Body message".into()),
        };
        assert_eq!(response, response_expected);
    }
    #[test]
    fn test_http_response_full() {
        let response = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 12\r\n\r\nBody message";
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_msg: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type:", "text/html");
                Some(h)
            },
            body: Some("Body message".into()),
        };
        let http_string: String = response_expected.into();
        assert_eq!(http_string, response);
    }
}

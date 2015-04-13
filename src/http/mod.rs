/// This HTTP client wrapper exists so that if hyper's http client becomes
/// unsupported, putting in a different client only required modifying this one
/// module.
///
extern crate hyper;

use std::option::Option;
use std::collections::HashMap;

// TODO
struct Response;

// TODO
enum HTTPError {
    Failed
}

struct Request<'a> {
    client: hyper::Client<hyper::net::HttpConnector<'a>>
}

impl<'a> Request<'a> {
    pub fn new() -> Request<'a> {
        Request { client: hyper::Client::new() }
    }

    pub fn exec(&self) -> Result<Response, HTTPError> {
        // TODO
        Err(HTTPError::Failed)
    }
}

pub struct Builder {
    method: Method,
    url: Option<String>,
    body: Option<String>,
    headers: HashMap<String, String>
}

impl<'a> Builder {
    fn new() -> Builder {
        Builder {
            method: Method::GET,
            url: None,
            body: None,
            headers: HashMap::new()
        }
    }

    fn method(&mut self, m: Method) -> &mut Builder {
        self.method = m;
        self
    }

    fn url(&mut self, url: &str) -> &mut Builder {
        self.url = Some(url.to_string());
        self
    }

    fn body(&mut self, body: &str) -> &mut Builder {
        self.body = Some(body.to_string());
        self
    }

    fn header(&mut self, header: &str, value: &str) -> &mut Builder {
        self.headers.insert(header.to_string(), value.to_string());
        self
    }

    fn build(&self) -> Request<'a> {
        // TODO set up request client
        Request::new()
    }
}

#[derive(Copy)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

impl Method {
    pub fn to_str(&self) -> &str {
        match *self {
            Method::GET => { "GET" },
            Method::POST => { "POST" },
            Method::PUT => { "PUT" },
            Method::DELETE => { "DELETE" }
        }
    }
}


#[test]
fn request_builder() {
    let req = Builder::new()
        .method(Method::PUT)
        .body("HELLO")
        .header("Content-Type", "application/json")
        .build();

    assert!(req.exec().is_err());
}

#[test]
fn make_request_direct() {
    let req = Request::new();
}

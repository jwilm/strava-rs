extern crate hyper;

use std::option::Option;
use std::collections::HashMap;

// use std::io::Read;

// use hyper::header::Connection;
// use hyper::header::ConnectionOption;
//
struct Response;
struct HTTPError;

struct Request<'a> {
    client: hyper::Client<hyper::net::HttpConnector<'a>>
}

impl<'a> Request<'a> {
    fn new() -> Request {
        Request { client: hyper::Client::new() }
    }

    fn exec() -> Result<Response, HTTPError> {
        Ok(Response);
    }
}

// struct Response;

pub struct Builder {
    method: Method,
    url: Option<String>,
    body: Option<String>,
    headers: HashMap<String, String>
}

impl Builder {
    fn new() -> Builder {
        Builder {
            method: Method::GET,
            url: None,
            body: None,
            headers: HashMap::new()
        }
    }

    fn method(&mut self, method: Method) -> &mut Builder {
        self.method = method;
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

    fn build(&self) -> Request {
        let req = Request::new();
    }
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

impl Method {
    fn to_str(&self) -> &str {
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

}

/// This HTTP client wrapper exists so that if hyper's http client becomes
/// unsupported, putting in a different client only required modifying this one
/// module.
///
extern crate hyper;

use std::option::Option;
use std::collections::HashMap;
use std::convert::From;
use std::result::Result;
use std::io::Read;

use std::fmt;

struct Response {
    body: String
}

impl Response {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }
}

use std::fmt::Display;

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str = self.body.as_ref();
        s.fmt(f)
    }
}

// TODO
#[derive(Debug)]
pub enum HttpError {
    Failed
}

impl From<hyper::error::Error> for HttpError {
    fn from(e: hyper::error::Error) -> HttpError {
        HttpError::Failed
    }
}

pub struct Http {
    body: Option<String>,
    headers: HashMap<String, String>
}

impl<'a> Http {
    pub fn new() -> Http {
        Http {
            body: None,
            headers: HashMap::new()
        }
    }

    pub fn body(&mut self, body: &str) -> &mut Http {
        self.body = Some(body.to_string());
        self
    }

    fn build(&self, method: Method, url: &str) -> Result<Response, HttpError> {
        let mut client = hyper::Client::new();

        let mut builder = match method {
            Method::GET => client.get(url),
            Method::PUT => client.put(url),
            Method::POST => client.post(url),
            Method::DELETE => client.delete(url)
        };

        // TODO is there a better way to do this?
        builder = if self.body.is_some() {
            let s: &str = (*(self.body.as_ref().unwrap())).as_ref();
            builder.body(s)
        } else {
            builder
        };

        let mut response = try!(builder.send());
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();

        Ok(Response {
            body: body
        })
    }

    pub fn get(&mut self, url: &str) -> Result<Response, HttpError> {
        self.build(Method::GET, url)
    }

    pub fn put(&mut self, url: &str) -> Result<Response, HttpError> {
        self.build(Method::PUT, url)
    }

    pub fn post(&mut self, url: &str) -> Result<Response, HttpError> {
        self.build(Method::POST, url)
    }

    pub fn delete(&mut self, url: &str) -> Result<Response, HttpError> {
        self.build(Method::DELETE, url)
    }
}

#[derive(Copy, Clone)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn request_wrapper_can_fetch() {
        let res = Http::new().get("http://www.google.com").unwrap();
        let body = res.body;
        assert!(body.contains("doctype html"));
    }
}

/// This HTTP client wrapper exists so that if hyper's http client becomes
/// unsupported, putting in a different client only required modifying this one
/// module.
///
extern crate hyper;

use std::fmt;
use std::io::Read;
use std::option::Option;
use std::result::Result;

use rustc_serialize::Decodable;
use rustc_serialize::json;

use hyper::status::StatusCode;

use error::ApiError;

pub struct Response {
    body: String,
    res: hyper::client::Response
}

impl Response {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }

    pub fn status(&self) -> hyper::status::StatusCode {
        self.res.status
    }
}

use std::fmt::Display;

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str = self.body.as_ref();
        s.fmt(f)
    }
}

pub type Error = hyper::error::Error;

pub struct Http {
    body: Option<String>
}

impl<'a> Http {
    pub fn new() -> Http {
        Http {
            body: None
        }
    }

    #[allow(dead_code)]
    pub fn body(&mut self, body: &str) -> &mut Http {
        self.body = Some(body.to_string());
        self
    }

    fn build(&self, method: Method, url: &str) -> Result<Response, Error> {
        let mut client = hyper::Client::new();

        let mut builder = match method {
            Method::GET => client.get(url),
            Method::PUT => client.put(url),
            Method::POST => client.post(url),
            Method::DELETE => client.delete(url)
        };

        // TODO is there a better way to do this?
        builder = if let Some(ref body) = self.body {
            builder.body(body)
        } else {
            builder
        };

        let mut response = try!(builder.send());
        let mut body = String::new();
        println!("{:?}", response);
        response.read_to_string(&mut body).unwrap();

        Ok(Response {
            body: body,
            res: response
        })
    }

    pub fn get(url: &str) -> Result<Response, Error> {
        Http::new().build(Method::GET, url)
    }

    #[allow(dead_code)]
    pub fn put(url: &str) -> Result<Response, Error> {
        Http::new().build(Method::PUT, url)
    }

    #[allow(dead_code)]
    pub fn post(url: &str) -> Result<Response, Error> {
        Http::new().build(Method::POST, url)
    }

    #[allow(dead_code)]
    pub fn delete(url: &str) -> Result<Response, Error> {
        Http::new().build(Method::DELETE, url)
    }
}

pub fn get<T>(url: &str) -> Result<T, ApiError> where T: Decodable {
    let response = try!(Http::get(url));
    match response.status() {
        StatusCode::Unauthorized => Err(ApiError::InvalidAccessToken),
        _ => Ok(try!(json::decode::<T>(response.body())))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

#[cfg(test)]
mod tests {
    use super::Http;

    #[test]
    fn request_wrapper_can_fetch() {
        let res = Http::get("http://www.google.com").unwrap();
        let body = res.body;
        assert!(body.contains("doctype html"));
    }
}

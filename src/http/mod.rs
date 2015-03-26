extern crate hyper;

// use std::io::Read;

// use hyper::header::Connection;
// use hyper::header::ConnectionOption;

struct Request;
// struct Response;

pub struct Client<'a> {
    client: hyper::Client<hyper::net::HttpConnector<'a>>
}

impl<'a> Client<'a> {
    pub fn new() -> Client<'a> {
        Client {
            client: hyper::Client::new()
        }
    }

//     pub fn get(&mut self, url: &str) -> Response {
//         let req = self.client.get(&url).send().unwrap();
//     }
}

#[test]
fn client_wrapper_works() {
    let client = Client::new();
}

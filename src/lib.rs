extern crate hyper;
extern crate time;

use std::io::Read;

// use hyper::Client;
// use hyper::header::Connection;
// use hyper::header::ConnectionOption;

mod http;
mod resources;

// #[test]
// fn it_works() {
//     let mut client = Client::new();
//     let mut res = client.get("http://www.google.com/")
//         .header(Connection(vec![ConnectionOption::Close]))
//         .send().unwrap();
// 
//     let mut body = String::new();
//     res.read_to_string(&mut body).unwrap();
//     println!("Response: {}", body);
// }

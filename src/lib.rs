#![feature(convert)]

//! Strava API
//!

extern crate hyper;
extern crate time;
// extern crate curl;

// use hyper::Client;
// use hyper::header::Connection;
// use hyper::header::ConnectionOption;

mod http;
pub mod resources;

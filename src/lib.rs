//! Strava API
//!

extern crate hyper;
extern crate rustc_serialize;

pub mod error;

pub mod api;
mod http;
mod map;
mod split;

pub mod athlete;

mod activity;
// pub use activity::Activity;
pub use activity::ActivityType;

pub mod segment;


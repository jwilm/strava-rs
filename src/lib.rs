//! A wrapper for the Strava API
//!
//! # Organization
//!
//! The module layout in this crate mirrors the [Strava API documentation][]. All functions
//! interacting with the strava servers will return a
//! [`strava::error::Result`](error/type.Result.html) for which the `E` parameter is curried to
//! [`strava::error::ApiError`](error/enum.ApiError.html).
//!
//! [Strava API Documentation]: http://strava.github.io/api/

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


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

/// Internal http api which currently wraps hyper
#[doc(hidden)]
mod http;

// Support modules
pub mod api;
pub mod error;
pub mod resources;

// Modules corresponding to strava docs
pub mod activities;
pub mod athletes;
pub mod clubs;
pub mod gear;
pub mod segmentefforts;
pub mod segments;
pub mod streams;
pub mod uploads;


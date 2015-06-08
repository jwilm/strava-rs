//! A wrapper for the Strava API
//!
//! # Organization
//!
//! The module layout in this crate mirrors the [Strava API documentation][]. All functions
//! interacting with the strava servers will return a
//! [`strava::error::Result`](error/type.Result.html) for which the `E` parameter is curried to
//! [`strava::error::ApiError`](error/enum.ApiError.html). The module listing below includes
//! **UNIMPLEMENTED** tags to make it clear what hasn't been written yet. Modules without that tag
//! are not guaranteed to exhaustively wrap all endpoints for that type, but they are guaranteed to
//! have support for between `1..all` endpoints for that type.
//!
//! # Examples
//!
//! The simplest thing one can do with the strava API is to get an athlete and print it out. Every
//! request made against the Strava API requires a valid access token. Since this example provides a
//! fake token, the `unwrap` call will panic with an `ApiError::InvalidAccessToken`. You can get a
//! token by registering an application on strava.com.
//!
//! ```no_run
//! use athletes::Athlete;
//! use api::AccessToken;
//!
//! // Create a token
//! let token = AccessToken::new("<my token>");
//!
//! // Get the athlete associated with the given token
//! let athlete = Athlete::get_current(&token).unwrap();
//!
//! // All of the strava types implement Debug and can be printed like so:
//! println!("{:?}", athlete);
//! ```
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


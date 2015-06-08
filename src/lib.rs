//! Strava API
//!

extern crate hyper;
extern crate rustc_serialize;

use std::default::Default;

use rustc_serialize::{Decodable, Decoder};

pub mod error;
pub mod paginate;

pub mod api;
mod http;
mod map;
mod split;

pub mod athlete;

mod accesstoken;
pub use accesstoken::AccessToken;

mod activity;
// pub use activity::Activity;
pub use activity::ActivityType;

pub mod segment;

/// Objects will be returned with a certain ResourceState
///
/// Detailed contains the most data and Meta the least.
#[allow(dead_code)]
#[derive(Debug, PartialEq, RustcEncodable)]
pub enum ResourceState {
    Unknown,
    Meta,
    Summary,
    Detailed
}

// TODO refactor primitive conversion into custom trait. Maybe add a macro or compiler plugin to
// handle this.
impl Decodable for ResourceState {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let num = try!(d.read_u8());
        Ok(match num {
            0 => ResourceState::Unknown,
            1 => ResourceState::Meta,
            2 => ResourceState::Summary,
            3 => ResourceState::Detailed,
            _ =>  unreachable!("ResourceState only valid for 0,1,2,3")
        })
    }
}

impl Default for ResourceState {
    fn default () -> ResourceState { ResourceState::Unknown }
}



#[cfg(test)]
mod tests {
    use std::default::Default;

    use super::ResourceState;

    #[test]
    fn resource_state_values() {
        assert_eq!(ResourceState::Meta as i32, 1);
        assert_eq!(ResourceState::Summary as i32, 2);
        assert_eq!(ResourceState::Detailed as i32, 3);
    }

    #[test]
    fn default_resource_state() {
        let default_state: ResourceState = Default::default();
        assert_eq!(default_state, ResourceState::Unknown);
    }
}

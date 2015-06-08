//! Helpers for the strava api
use std::env;
use std::convert::From;

use rustc_serialize::{Decodable, Decoder};

#[doc(hidden)]
pub fn v3(token: &AccessToken, url: String) -> String {
    format!("https://www.strava.com/api/v3/{}?access_token={}", url, token.get())
}

/// Wrapper for endpoints that paginate
///
/// A Paginated<T> will be returned from any endpoint that supports paging. Provides methods for
/// fetching the next page and checking if more pages are available.
#[derive(Debug)]
pub struct Paginated<T> {
    page: usize,
    per_page: usize,
    url: String,
    data: Vec<T>
}

impl<T> Paginated<T> {
    pub fn new(url: String, data: Vec<T>) -> Paginated<T> {
        Paginated {
            page: 1,
            per_page: 30,
            url: url,
            data: data,
        }
    }

    /// Get the next page of results
    pub fn fetch_next_page(&self) -> Option<Paginated<T>> {
        unimplemented!();
    }

    /// Check if this is the last page
    pub fn last_page(&self) -> bool {
        self.per_page != self.data.len()
    }
}

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

/// A strava.com api access token.
///
/// You'll need to register/login at https://www.strava.com/developers to get a token. This is
/// required for all requests.
pub struct AccessToken {
    token: String
}

impl AccessToken {
    /// Create an AccessToken from the supplied string
    pub fn new(token: String) -> AccessToken {
        AccessToken {
            token: token
        }
    }

    /// Create an AccessToken from the environment variable STRAVA_ACCESS_TOKEN
    pub fn new_from_env() -> Result<AccessToken, env::VarError> {
        match env::var("STRAVA_ACCESS_TOKEN") {
            Ok(token) => Ok(AccessToken::new(token)),
            Err(e) => Err(e)
        }
    }

    /// Get the token underlying string
    ///
    /// This is used internally for building requests.
    // TODO implement Deref -> &str for AccessToken
    pub fn get(&self) -> &str {
        &self.token[..]
    }
}

impl<'a> From<&'a str> for AccessToken {
    fn from(s: &'a str) -> AccessToken {
        AccessToken { token: s.to_string() }
    }
}

#[cfg(test)]
mod resource_state_tests {
    use std::default::Default;

    use super::ResourceState;

    #[test]
    fn values() {
        assert_eq!(ResourceState::Meta as i32, 1);
        assert_eq!(ResourceState::Summary as i32, 2);
        assert_eq!(ResourceState::Detailed as i32, 3);
    }

    #[test]
    fn default() {
        let default_state: ResourceState = Default::default();
        assert_eq!(default_state, ResourceState::Unknown);
    }
}

#[cfg(test)]
mod paginated_tests {
    use super::Paginated;

    #[test]
    fn last_page() {
        let vec = (0..30).collect::<Vec<u8>>();
        let pager = Paginated::new("test".to_string(), vec);
        println!("{:?}", pager);
        assert_eq!(pager.last_page(), false);
    }
}

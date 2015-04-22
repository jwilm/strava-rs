use std::default::Default;
use std::option::Option;

use time::Timespec;

// use resources::Club;
// use resources::Gear;
use resources::enums::ResourceState;

use rustc_serialize::json;
use rustc_serialize::json::DecoderError;

use http::{Http, HttpError};
use accesstoken::AccessToken;

/// Athletes are Strava users, Strava users are athletes.
///
/// The object is returned in detailed, summary or meta representations.
///
/// See: http://strava.github.io/api/v3/athlete/
#[allow(dead_code)]
#[derive(Default, RustcEncodable, RustcDecodable, Debug)]
pub struct Athlete {
    id: Option<i32>,
    resource_state: ResourceState,
    firstname: Option<String>,
    lastname: Option<String>,
    profile_medium: Option<String>,
    profile: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    sex: Option<String>,
    friend: Option<String>,
    follower: Option<String>,
    premium: Option<bool>,
    created_at: Option<String>,
    updated_at: Option<String>,
    approve_followers: Option<bool>,
    follower_count: Option<i32>,
    friend_count: Option<i32>,
    mutual_friend_count: Option<i32>,
    date_preference: Option<String>,
    measurement_preference: Option<String>,
    email: Option<String>,
    ftp: Option<i32>,
    weight: Option<f32>
    // clubs: Vec<Club>,
    // shoes: Vec<Gear>,
    // bikes: Vec<Gear>
}

#[derive(Debug)]
pub enum ApiError {
    InvalidAccessToken,
    EmptyResponse,
    NetworkError,
    InvalidJson(DecoderError)
}

impl From<DecoderError> for ApiError {
    fn from(e: DecoderError) -> ApiError {
        ApiError::InvalidJson(e)
    }
}

impl From<HttpError> for ApiError {
    fn from(e: HttpError) -> ApiError {
        ApiError::NetworkError
    }
}

impl Athlete {
    fn new() -> Athlete { Default::default() }

    pub fn get_current(token: &AccessToken) -> Result<Athlete, ApiError> {
        let url = format!("https://strava.com/api/v3/athlete?access_token={}", token.get());

        let response = try!(Http::new().get(url.as_ref()));
        let athlete = try!(json::decode::<Athlete>(response.body()));
        Ok(athlete)
    }

    pub fn get_by_id(id: i32) -> Result<Athlete, ApiError> {
        let url = format!("https://strava.com/api/v3/athletes/{}", id);
        println!("{}", url);

        // TODO make http request
        Ok(Athlete::new())
    }
}

#[cfg(test)]
mod tests {
    use accesstoken::AccessToken;
    use resources::athlete::Athlete;
    use std::result::Result;
    use resources::enums::ResourceState;

    #[test]
    fn get_current_athlete() {
        let token = AccessToken::from("<fake token replace me>");
        let athlete: Athlete = Athlete::get_current(&token).unwrap();
        assert!(athlete.resource_state == ResourceState::Detailed);
    }
}

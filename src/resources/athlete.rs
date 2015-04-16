use std::default::Default;
use std::option::Option;

use time::Timespec;

// use resources::Club;
// use resources::Gear;
use resources::enums::ResourceState;

use http::Http;
use accesstoken::AccessToken;

/// Athletes are Strava users, Strava users are athletes.
///
/// The object is returned in detailed, summary or meta representations.
///
/// See: http://strava.github.io/api/v3/athlete/
#[allow(dead_code)]
#[derive(Default)]
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
    created_at: Option<Timespec>,
    updated_at: Option<Timespec>,
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

#[derive(Debug, Copy, Clone)]
pub enum ApiError {
    InvalidAccessToken
}

impl Athlete {
    fn new() -> Athlete { Default::default() }

    pub fn get_current(token: &AccessToken) -> Result<Athlete, ApiError> {
        let url = format!("https://strava.com/api/v3/athlete?access_token={}", token.get());
        println!("access token: {}", url);
        let res = Http::new().get(url.as_ref()).unwrap();

        println!("{}", res);

        // TODO make http request
        Ok(Athlete::new())
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

    #[test]
    fn get_current_athlete() {
        let token = AccessToken::from("fake_token");

        let res = Athlete::get_current(&token);
        assert!(!res.is_err());
        assert!(res.is_err());
    }
}

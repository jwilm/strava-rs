use std::option::Option;

// use resources::Club;
// use resources::Gear;
use resources::enums::ResourceState;

use http;
use accesstoken::AccessToken;

use error::ApiError;

/// Athletes are Strava users, Strava users are athletes.
///
/// The object is returned in detailed, summary or meta representations.
///
/// See: http://strava.github.io/api/v3/athlete/
#[allow(dead_code)]
#[derive(RustcDecodable, Debug)]
pub struct Athlete {
    pub id: i32,
    pub resource_state: ResourceState,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub profile_medium: Option<String>,
    pub profile: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub sex: Option<String>,
    pub friend: Option<String>,
    pub follower: Option<String>,
    pub premium: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub approve_followers: Option<bool>,
    pub follower_count: Option<i32>,
    pub friend_count: Option<i32>,
    pub mutual_friend_count: Option<i32>,
    pub date_preference: Option<String>,
    pub measurement_preference: Option<String>,
    pub email: Option<String>,
    pub ftp: Option<i32>,
    pub weight: Option<f32>
    // pub clubs: Vec<Club>,
    // pub shoes: Vec<Gear>,
    // pub bikes: Vec<Gear>
}


impl Athlete {
    pub fn get_current(token: &AccessToken) -> Result<Athlete, ApiError> {
        let url = format!("https://strava.com/api/v3/athlete?access_token={}", token.get());
        http::get::<Athlete>(&url[..])
    }

    pub fn get(token: &AccessToken, id: i32) -> Result<Athlete, ApiError> {
        let url = format!("https://strava.com/api/v3/athletes/{}?access_token={}", id, token.get());
        http::get::<Athlete>(&url[..])
    }
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use accesstoken::AccessToken;
    use super::Athlete;
    use resources::enums::ResourceState;

    #[test]
    fn get_current_athlete() {
        let token = AccessToken::new_from_env().unwrap();
        let athlete: Athlete = Athlete::get_current(&token).unwrap();
        assert_eq!(athlete.resource_state, ResourceState::Detailed);
    }

    #[test]
    fn get_athlete_by_id() {
        let id = 1712082;
        let token = AccessToken::new_from_env().unwrap();
        let athlete = Athlete::get(&token, id).unwrap();
        println!("{:?}", athlete);
        assert_eq!(athlete.id, id);
    }
}

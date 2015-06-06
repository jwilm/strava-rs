use std::option::Option;

// use resources::Club;
// use resources::Gear;
use resources::enums::ResourceState;

use http;
use accesstoken::AccessToken;

use error::Result;

/// Athletes are Strava users, Strava users are athletes.
///
/// The object is returned in detailed, summary or meta representations.
///
/// See: http://strava.github.io/api/v3/athlete/
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

/// Statistics for an athlete
///
/// This data is only retrieveable for the currently authenticated athlete. Values are in seconds
/// and meters.
///
/// http://strava.github.io/api/v3/athlete/#stats
#[derive(Debug, RustcDecodable)]
pub struct Stats {
    biggest_ride_distance: f32,
    biggest_climb_elevation_gain: f32,
    recent_ride_totals: RecentTotals,
    recent_run_totals: RecentTotals,
    ytd_ride_totals: Totals,
    ytd_run_totals: Totals,
    all_ride_totals: Totals,
    all_run_totals: Totals,
}

#[derive(Debug, RustcDecodable)]
pub struct RecentTotals {
    count: i32,
    distance: f32,
    moving_time: i32,
    elapsed_time: i32,
    elevation_gain: f32,
    achievement_count: i32
}

#[derive(Debug, RustcDecodable)]
pub struct Totals {
    count: i32,
    distance: i32,
    moving_time: i32,
    elapsed_time: i32,
    elevation_gain: i32
}



impl Athlete {
    pub fn get_current(token: &AccessToken) -> Result<Athlete> {
        let url = format!("https://strava.com/api/v3/athlete?access_token={}", token.get());
        http::get::<Athlete>(&url[..])
    }

    pub fn get(token: &AccessToken, id: i32) -> Result<Athlete> {
        let url = format!("https://strava.com/api/v3/athletes/{}?access_token={}", id, token.get());
        http::get::<Athlete>(&url[..])
    }

    pub fn stats(&self, token: &AccessToken) -> Result<Stats> {
        let url = format!("https://strava.com/api/v3/athletes/{}/stats?access_token={}",
                          self.id, token.get());
        http::get::<Stats>(&url[..])
    }
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use accesstoken::AccessToken;
    use super::Athlete;
    use resources::enums::ResourceState;
    use error::ApiError;

    #[test]
    fn get_current_athlete() {
        let token = AccessToken::new_from_env().unwrap();
        let athlete = Athlete::get_current(&token).unwrap();
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

    #[test]
    #[allow(dead_code)]
    fn get_athlete_stats() {
        let token = AccessToken::new_from_env().unwrap();
        let athlete = Athlete::get_current(&token).unwrap();
        let stats = athlete.stats(&token).unwrap();

        println!("{:?}", stats);
    }

    #[test]
    fn get_other_athlete_stats() {
        let id = 1712082;
        let token = AccessToken::new_from_env().unwrap();
        let athlete = Athlete::get(&token, id).unwrap();
        match athlete.stats(&token) {
            Ok(_) => panic!("somehow got stats for other athlete"),
            Err(ApiError::InvalidAccessToken) => (),
            Err(e) => panic!("unexpected error type {:?}", e)
        }
    }
}

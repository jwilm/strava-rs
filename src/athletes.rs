//! Strava athletes and associated data
use std::option::Option;

use api::{self, Paginated, AccessToken, ResourceState};
use error::Result;
use http;
use gear::Gear;
use clubs::Club;
use segmentefforts::SegmentEffort;

/// A strava athlete
///
/// The object may be returned in detailed, summary or meta representations.
///
/// See: http://strava.github.io/api/v3/athlete/
#[derive(Debug, RustcDecodable)]
pub struct Athlete {
    /// Athlete's ID on strava
    pub id: i32,
    /// The resource state dictates which fields should be safe to unwrap
    pub resource_state: ResourceState,
    /// First name
    pub firstname: Option<String>,
    /// Last name
    pub lastname: Option<String>,
    /// URL to a 62x62 pixel profile picture
    pub profile_medium: Option<String>,
    /// URL to a 124x124 pixel profile picture
    pub profile: Option<String>,
    /// home city
    pub city: Option<String>,
    /// home state
    pub state: Option<String>,
    /// home country
    pub country: Option<String>,
    /// gender
    pub sex: Option<String>,
    /// "pending", "accepted", "blocked" or None - the authenticated athlete’s following status
    /// of this athlete
    pub friend: Option<String>,
    /// "pending", "accepted", "blocked" or None - this athlete's following status of the
    /// authenticated athlete
    pub follower: Option<String>,
    /// The athlete uses strava's premium features
    pub premium: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub approve_followers: Option<bool>,
    pub follower_count: Option<i32>,
    pub friend_count: Option<i32>,
    pub mutual_friend_count: Option<i32>,
    pub date_preference: Option<String>,
    /// feet or meters
    pub measurement_preference: Option<String>,
    pub email: Option<String>,
    pub ftp: Option<i32>,
    pub weight: Option<f32>,
    pub athlete_type: u32,
    pub clubs: Vec<Club>,
    //gear
    pub shoes: Vec<Gear>,
    pub bikes: Vec<Gear>
}

/// Statistics for an athlete
///
/// This data is only retrieveable for the currently authenticated athlete. Values are in seconds
/// and meters.
///
/// http://strava.github.io/api/v3/athlete/#stats
#[derive(Debug, RustcDecodable)]
pub struct Stats {
    pub biggest_ride_distance: f32,
    pub biggest_climb_elevation_gain: f32,
    pub recent_ride_totals: RecentTotals,
    pub recent_run_totals: RecentTotals,
    pub ytd_ride_totals: Totals,
    pub ytd_run_totals: Totals,
    pub all_ride_totals: Totals,
    pub all_run_totals: Totals,
}

/// Total statistics for a recent time period
#[derive(Debug, RustcDecodable)]
pub struct RecentTotals {
    pub count: i32,
    pub distance: f32,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub elevation_gain: f32,
    pub achievement_count: i32
}

/// Total statistics for an arbitrary time period
#[derive(Debug, RustcDecodable)]
pub struct Totals {
    pub count: i32,
    pub distance: i32,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub elevation_gain: i32
}

impl Athlete {
    /// Get the athlete associated with the given access token
    pub fn get_current(token: &AccessToken) -> Result<Athlete> {
        let url = api::v3(token, "athlete".to_string());
        http::get::<Athlete>(&url[..])
    }

    /// Get an Athlete by id
    pub fn get(token: &AccessToken, id: i32) -> Result<Athlete> {
        let url = api::v3(token, format!("athletes/{}", id));
        http::get::<Athlete>(&url[..])
    }

    /// Get stats for an athlete.
    /// This is only available for the currently authenticated athlete
    pub fn stats(&self, token: &AccessToken) -> Result<Stats> {
        let url = api::v3(token, format!("athletes/{}/stats", self.id));
        http::get::<Stats>(&url[..])
    }

    /// Get all KOMs for the Athlete.
    pub fn koms(&self, token: &AccessToken) -> Result<Paginated<SegmentEffort>> {
        let url = api::v3(token, format!("athletes/{}/koms", self.id));
        let efforts = try!(http::get::<Vec<SegmentEffort>>(&url[..]));
        Ok(Paginated::new(url, efforts))
    }
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use api::{AccessToken, ResourceState};
    use super::Athlete;
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

    #[test]
    fn get_koms() {
        let token = AccessToken::new_from_env().unwrap();
        let athlete = Athlete::get_current(&token).unwrap();
        let koms = athlete.koms(&token).unwrap();
        println!("{:?}", koms);
    }

    #[test]
    fn print_response() {
        let token = AccessToken::new_from_env().unwrap();
        let athlete = Athlete::get_current(&token).unwrap();
        println!("{:?}",athlete);
    }
}

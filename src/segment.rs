//! Specific sections of road and attempts an athlete has made on them
use activity::ActivityType;
use athlete::Athlete;
use api::{self, Paginated, AccessToken, ResourceState};
use error::Result;
use http;

/// A specific section(s) of road.
///
/// Segments are available in Summary and Detail versions.
/// http://strava.github.io/api/v3/segments/
#[derive(Debug, RustcDecodable)]
pub struct Segment {
    id: u32,
    resource_state: ResourceState,
    name: String,
    activity_type: ActivityType,
    distance: f32,
    average_grade: f32,
    maximum_grade: f32,
    elevation_high: f32,
    elevation_low: f32,
    start_latlng: Vec<f32>,
    end_latlng: Vec<f32>,
    climb_category: u8,
    city: String,
    state: String,
    country: String,
    private: bool,

    // Detail Attributes
    created_at: Option<String>,
    updated_at: Option<String>,
    total_elevation_gain: Option<f32>,
    // TODO map: Option<Map>,
    effort_count: Option<u32>,
    athlete_count: Option<u32>,
    star_count: Option<u32>,
    hazardous: Option<bool>

    // Properties on starred segments
    // TODO atlete_pr_effort: Effort,
}

impl Segment {
    /// Fetch a Segment by id
    pub fn get(token: &AccessToken, id: u32) -> Result<Segment> {
        let url = api::v3(token, format!("segments/{}", id));
        Ok(try!(http::get::<Segment>(&url[..])))
    }

    /// Get starred segments
    pub fn get_starred(token: &AccessToken) -> Result<Paginated<Segment>> {
        let url = api::v3(token, "segments/starred".to_string());
        let segments = try!(http::get::<Vec<Segment>>(&url[..]));
        Ok(Paginated::new(url, segments))
    }
}

/// An athlete's attempt at a segment (the portion of a ride that covers a segment)
///
/// Available in summary and detail representations, but they are the same at this time.
///
/// http://strava.github.io/api/v3/efforts/#retrieve
#[derive(Debug, RustcDecodable)]
pub struct Effort {
    id: i64,
    resource_state: ResourceState,
    name: String,
    // TODO activity: Activity, // Meta representation only
    athlete: Athlete,           // Meta representation only
    elapsed_time: u32,
    moving_time: u32,
    start_date: String,
    start_date_local: String,
    distance: f32,
    start_index: u32,
    end_index: u32,
    average_cadence: f32,
    average_watts: f32,
    device_watts: bool,
    average_heartrate: f32,
    max_heartrate: f32,
    segment: Segment,           // Summary representation
    kom_rank: Option<u8>,
    pr_rank: Option<u8>
}

impl Effort {
    /// List efforts for the given segment ID
    ///
    /// http://strava.github.io/api/v3/segments/#efforts
    ///
    /// TODO support filtering by athlete
    /// TODO support filtering by date range
    pub fn list_for_segment(token: &AccessToken, segment_id: u32) -> Result<Paginated<Effort>> {
        let url = api::v3(token, format!("segments/{}/all_efforts", segment_id));
        let efforts = try!(http::get::<Vec<Effort>>(&url[..]));
        Ok(Paginated::new(url, efforts))
    }
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Segment;
    use super::Effort;
    use api::AccessToken;

    #[test]
    fn get_segment() {
        let token = AccessToken::new_from_env().unwrap();
        let segment = Segment::get(&token, 646257).unwrap();

        println!("{:?}", segment);
    }

    #[test]
    fn get_stars() {
        let token = AccessToken::new_from_env().unwrap();
        let starred = Segment::get_starred(&token).unwrap();
        println!("{:?}", starred);
    }

    #[test]
    fn get_efforts_for_segment() {
        let token = AccessToken::new_from_env().unwrap();
        let pager = Effort::list_for_segment(&token, 646257).unwrap();
        println!("{:?}", pager);
    }
}

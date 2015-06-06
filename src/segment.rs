use resources::enums::ResourceState;
use resources::enums::ActivityType;
use error::Result;
use http;
use accesstoken::AccessToken;
use paginate::Paginated;

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
        let url = format!("https://www.strava.com/api/v3/segments/{}?access_token={}",
                          id, token.get());
        Ok(try!(http::get::<Segment>(&url[..])))
    }

    /// Get starred segments
    pub fn get_starred(token: &AccessToken) -> Result<Paginated<Segment>> {
        let url = format!("https://www.strava.com/api/v3/segments/starred?access_token={}",
                          token.get());
        let segments = try!(http::get::<Vec<Segment>>(&url[..]));
        Ok(Paginated::new(url, segments))
    }
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Segment;
    use accesstoken::AccessToken;

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
}

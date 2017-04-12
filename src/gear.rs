use api::{ResourceState, AccessToken};
use error::Result;

/// Gear type able to represent bikes/shoes/etc.
///
/// Represents equipment used during an activity. Gear has summary and detail
/// representations.
///
/// See: http://strava.github.io/api/v3/gear/
#[derive(Debug, RustcDecodable)]
pub struct Gear {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub distance: f32,
    pub resource_state: ResourceState,
    pub brand_name: Option<String>,
    pub model_name: Option<String>,
    pub frame_type: Option<FrameType>,
    pub description: Option<String>,
}

impl Gear {
    pub fn get(token: &AccessToken, id: String) -> Result<String>{
        Ok("test".to_string())
    }
}

/// Frame type for bikes
#[derive(Debug, RustcDecodable)]
pub enum FrameType {
    MTB,
    Cross,
    Road,
    TimeTrial
}

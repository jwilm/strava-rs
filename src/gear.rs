use api::ResourceState;

/// Gear type able to represent bikes/shoes/etc.
///
/// Represents equipment used during an activity. Gear has summary and detail
/// representations.
///
/// See: http://strava.github.io/api/v3/gear/
pub struct Gear {
    pub id: String,
    pub primary: bool,
    pub name: String,
    pub distance: f32,
    pub brand_name: String,
    pub model_name: String,
    pub frame_type: FrameType,
    pub description: String,
    pub resource_state: ResourceState
}

/// Frame type for bikes
#[derive(Debug)]
pub enum FrameType {
    MTB,
    Cross,
    Road,
    TimeTrial
}

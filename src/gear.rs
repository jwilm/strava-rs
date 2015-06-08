/// Gear type able to represent bikes/shoes/etc.
///
/// Represents equipment used during an activity. Gear has summary and detail
/// representations.
///
/// See: http://strava.github.io/api/v3/gear/
#[allow(dead_code)]
pub struct Gear {
    id: String,
    primary: bool,
    name: String,
    distance: f32,
    brand_name: String,
    model_name: String,
    frame_type: FrameType,
    description: String,
    resource_state: ResourceState
}

/// Frame type for bikes
#[derive(Debug)]
pub enum FrameType {
    MTB,
    Cross,
    Road,
    TimeTrial
}

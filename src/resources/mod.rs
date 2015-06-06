//! Strava resource types for API v3
//!
//! Most member types are a 1:1 mapping except enumerations, resources, and
//! times
//!

// use time::Timespec;

pub mod enums;
pub use self::enums::ResourceState;
use self::enums::FrameType;
use self::enums::ActivityType;
use self::enums::ClubType;
use self::enums::SportType;
// use self::enums::WorkoutType;

pub mod athlete;
pub use self::athlete::Athlete;

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



/// Clubs represent groups of athletes on Strava.
///
/// They can be public or private. Only members of private clubs can access
/// their details. The object is returned in summary or detailed
/// representations.
///
/// See: http://strava.github.io/api/v3/clubs/
#[allow(dead_code)]
pub struct Club {
    id: i32,
    resource_state: ResourceState,
    name: String,
    profile_medium: String,
    profile: String,
    description: String,
    club_type: ClubType,
    sport_type: SportType,
    city: String,
    state: String,
    country: String,
    private: bool,
    member_count: i32
}

pub struct ActivityMap;


pub struct SegmentEffort;
pub struct Split;

#[allow(dead_code)]
pub struct Activity {
    id: i32,
    resource_state: ResourceState,
    external_id: String,
    upload_id: i32,
    athlete: Athlete,
    name: String,
    description: String,
    distance: f32,
    moving_time: i32,
    elapsed_time: i32,
    total_elevation_gain: f32,
    activity_type: ActivityType,
    // start_date: Timespec,
    // start_date_local: Timespec,
    timezone: String,
    start_latlng: (f32, f32),
    end_latlng: (f32, f32),
    location_city: String,
    location_state: String,
    location_country: String,
    achievement_count: i32,
    kudos_count: i32,
    comment_count: i32,
    athlete_count: i32,
    photo_count: i32,
    map: ActivityMap,
    trainer: bool,
    commute: bool,
    manual: bool,
    private: bool,
    flagged: bool,
    workout_type: i32,
    gear_id: String,
    gear: Gear,
    average_speed: f32,
    max_speed: f32,
    average_cadence: f32,
    average_temp: f32,
    average_watts: f32,
    weighted_average_watts: i32,
    kilojoules: f32,
    device_watts: bool,
    max_heartrate: i32,
    calories: f32,
    truncated: i32,
    has_kudoed: bool,
    segment_efforts: Vec<SegmentEffort>,
    splits_metric: Vec<Split>,
    splits_standard: Vec<Split>,
    best_efforts: Vec<SegmentEffort>
}



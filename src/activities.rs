//! **UNIMPLEMENTED:** Base object for Strava runs, rides, swims etc.
//!
//! Although there are a couple of types defined here, no endpoints are currently accessible with
//! this library.
use rustc_serialize::{Decodable, Decoder};

use api::ResourceState;
use athletes::Athlete;
use segmentefforts::SegmentEffort;
use resources;

/// Activity Types
#[derive(Debug)]
pub enum ActivityType {
    Ride,
    Run,
    Swim,
    Hike,
    Walk,
    AlpineSki,
    BackcountrySki,
    Canoeing,
    CrossCountrySkiing,
    Crossfit,
    Elliptical,
    IceSkate,
    InlineSkate,
    Kayaking,
    Kitesurf,
    NordicSki,
    RockClimbing,
    RollerSki,
    Rowing,
    Snowboard,
    Snowshoe,
    StairStepper,
    StandUpPaddling,
    Surfing,
    WeightTraining,
    Windsurf,
    Workout,
    Yoga,
    Unknown
}

impl Decodable for ActivityType {
    fn decode<D: Decoder>(d: &mut D) -> Result<ActivityType, D::Error> {
        d.read_str().map(|s| {
            match &s[..] {
                "Ride" => { ActivityType::Ride },
                "Run" => { ActivityType::Run },
                "Swim" => { ActivityType::Swim },
                "Hike" => { ActivityType::Hike },
                "Walk" => { ActivityType::Walk },
                "AlpineSki" => { ActivityType::AlpineSki },
                "BackcountrySki" => { ActivityType::BackcountrySki },
                "Canoeing" => { ActivityType::Canoeing },
                "CrossCountrySkiing" => { ActivityType::CrossCountrySkiing },
                "Crossfit" => { ActivityType::Crossfit },
                "Elliptical" => { ActivityType::Elliptical },
                "IceSkate" => { ActivityType::IceSkate },
                "InlineSkate" => { ActivityType::InlineSkate },
                "Kayaking" => { ActivityType::Kayaking },
                "Kitesurf" => { ActivityType::Kitesurf },
                "NordicSki" => { ActivityType::NordicSki },
                "RockClimbing" => { ActivityType::RockClimbing },
                "RollerSki" => { ActivityType::RollerSki },
                "Rowing" => { ActivityType::Rowing },
                "Snowboard" => { ActivityType::Snowboard },
                "Snowshoe" => { ActivityType::Snowshoe },
                "StairStepper" => { ActivityType::StairStepper },
                "StandUpPaddling" => { ActivityType::StandUpPaddling },
                "Surfing" => { ActivityType::Surfing },
                "WeightTraining" => { ActivityType::WeightTraining },
                "Windsurf" => { ActivityType::Windsurf },
                "Workout" => { ActivityType::Workout },
                "Yoga" => { ActivityType::Yoga }
                _ => { ActivityType::Unknown }
            }
        })
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum WorkoutType {
    Default,
    Race,
    Long,
    Intervals
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Activity {
    // Meta representation
    pub id: i32,
    pub resource_state: ResourceState,

    // Summary representation
    pub external_id: String,
    pub upload_id: i32,
    pub athlete: Athlete,
    pub name: String,
    pub distance: f32,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub total_elevation_gain: f32,
    pub activity_type: ActivityType,
    pub start_date: String,
    pub start_date_local: String,
    // TODO pub start_date: Timespec,
    // TODO pub start_date_local: Timespec,
    pub timezone: String,
    pub start_latlng: (f32, f32),
    pub end_latlng: (f32, f32),
    pub location_city: String,
    pub location_state: String,
    pub location_country: String,
    pub achievement_count: i32,
    pub kudos_count: i32,
    pub comment_count: i32,
    pub athlete_count: i32,
    pub photo_count: i32,
    pub map: resources::Map,
    pub trainer: bool,
    pub commute: bool,
    pub manual: bool,
    pub private: bool,
    pub flagged: bool,
    pub workout_type: i32,
    pub gear_id: String,
    pub average_speed: f32,
    pub max_speed: f32,
    pub average_cadence: f32,
    pub average_temp: f32,
    pub average_watts: f32,
    pub weighted_average_watts: i32,
    pub kilojoules: f32,
    pub device_watts: bool,
    pub max_heartrate: i32,
    pub truncated: i32,
    pub has_kudoed: bool,

    // Detail represenation
    pub calories: f32,
    pub description: String,
    // TODO pub gear: Gear,
    pub segment_efforts: Vec<SegmentEffort>,
    pub splits_metric: Vec<Split>,
    pub splits_standard: Vec<Split>,
    pub best_efforts: Vec<SegmentEffort>
    // TODO pub photos: Photos
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Split;

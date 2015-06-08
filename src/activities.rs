use rustc_serialize::{Decodable, Decoder};

use api::ResourceState;
use athletes::Athlete;
use segmentefforts::SegmentEffort;
use resources::Map;

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
    map: Map,
    trainer: bool,
    commute: bool,
    manual: bool,
    private: bool,
    flagged: bool,
    workout_type: i32,
    gear_id: String,
    // TODO gear: Gear,
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Split;

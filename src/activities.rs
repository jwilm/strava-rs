//endpoint for reading activities 
use error::Result;

use api::{self, ResourceState, AccessToken};
use http;
use athletes::Athlete;
use segmentefforts::SegmentEffort;
use resources;

/// Activity Types
#[derive(Debug,RustcDecodable)]
pub enum ActivityType {
    Ride,
    Run,
    Swim,
    Hike,
    Walk,
    AlpineSki,
    BackcountrySki,
    Canoeing,
    Crossfit,
    EBikeRide,
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

#[derive(Debug,RustcDecodable)]
pub enum WorkoutType {
    DefaultRun = 0,
    RaceRun = 1,
    LongRun = 2,
    WorkoutRun = 3,
    DefaultRide = 10,
    RaceRide = 11,
    WorkoutRide = 12,
}

#[derive(Debug,RustcDecodable)]
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
    pub start_date: String, //TODO decode time from string 
    pub start_date_local: String, //TODO decode time from string 
    pub timezone: String,
    pub start_latlng: Cords,
    pub end_latlng: Cords,
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
    pub workout_type: WorkoutType,
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

#[derive(Debug,RustcDecodable)]
pub struct Cords{
    x: f32,
    y: f32,
}

impl Activity {
    /// Get an Gear by id (the only option)
    pub fn get(token: &AccessToken, id: String) -> Result <Activity> {
        let url = api::v3(token, format!("activities/{}", id));
        http::get::<Activity>(&url[..])
    }
}

#[allow(dead_code)]
#[derive(Debug,RustcDecodable)]
pub struct Split;

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Activity;
    use api::AccessToken;
    #[test]
    fn get_activity() {
        let id = "321934".to_string();
        let token = AccessToken::new_from_env().unwrap();
        let activity = Activity::get(&token,id);
        println!("{:?}",activity);
    }
}
use std::str::FromStr;
use std::default::Default;

/// Objects will be returned with a certain ResourceState
///
/// Detailed contains the most data and Meta the least.
#[allow(dead_code)]
#[derive(Debug, FromPrimitive, PartialEq)]
pub enum ResourceState {
    Unknown,
    Meta,
    Summary,
    Detailed
}

impl Default for ResourceState {
    fn default () -> ResourceState { ResourceState::Unknown }
}

/// Frame type for bikes
#[allow(dead_code)]
#[derive(Debug, FromPrimitive)]
pub enum FrameType {
    MTB,
    Cross,
    Road,
    TimeTrial
}

/// Activity Types
#[allow(dead_code)]
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
    Yoga
}

impl ActivityType {
    fn string(&self) -> String {
        format!("{:?}", &self)
    }
}

pub enum ValueError {
    InvalidValue,
    Other
}

impl FromStr for ActivityType {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<ActivityType, ValueError> {
        match s {
            "Ride" => { Ok(ActivityType::Ride) },
            "Run" => { Ok(ActivityType::Run) },
            "Swim" => { Ok(ActivityType::Swim) },
            "Hike" => { Ok(ActivityType::Hike) },
            "Walk" => { Ok(ActivityType::Walk) },
            "AlpineSki" => { Ok(ActivityType::AlpineSki) },
            "BackcountrySki" => { Ok(ActivityType::BackcountrySki) },
            "Canoeing" => { Ok(ActivityType::Canoeing) },
            "CrossCountrySkiing" => { Ok(ActivityType::CrossCountrySkiing) },
            "Crossfit" => { Ok(ActivityType::Crossfit) },
            "Elliptical" => { Ok(ActivityType::Elliptical) },
            "IceSkate" => { Ok(ActivityType::IceSkate) },
            "InlineSkate" => { Ok(ActivityType::InlineSkate) },
            "Kayaking" => { Ok(ActivityType::Kayaking) },
            "Kitesurf" => { Ok(ActivityType::Kitesurf) },
            "NordicSki" => { Ok(ActivityType::NordicSki) },
            "RockClimbing" => { Ok(ActivityType::RockClimbing) },
            "RollerSki" => { Ok(ActivityType::RollerSki) },
            "Rowing" => { Ok(ActivityType::Rowing) },
            "Snowboard" => { Ok(ActivityType::Snowboard) },
            "Snowshoe" => { Ok(ActivityType::Snowshoe) },
            "StairStepper" => { Ok(ActivityType::StairStepper) },
            "StandUpPaddling" => { Ok(ActivityType::StandUpPaddling) },
            "Surfing" => { Ok(ActivityType::Surfing) },
            "WeightTraining" => { Ok(ActivityType::WeightTraining) },
            "Windsurf" => { Ok(ActivityType::Windsurf) },
            "Workout" => { Ok(ActivityType::Workout) },
            "Yoga" => { Ok(ActivityType::Yoga) }
            _ => { Err(ValueError::InvalidValue) }
        }
    }
}

/// Types of clubs
#[allow(dead_code)]
#[derive(Debug, FromPrimitive)]
pub enum ClubType {
    Casual,
    Racing,
    Triathlon,
    Other
}

/// Types of sports
#[allow(dead_code)]
#[derive(Debug, FromPrimitive)]
pub enum SportType {
    Cycling,
    Running,
    Triathlon,
    Other
}

#[allow(dead_code)]
#[derive(Debug, FromPrimitive)]
pub enum WorkoutType {
    Default,
    Race,
    Long,
    Intervals
}

#[test]
fn activity_type_to_string() {
    assert_eq!("Ride", ActivityType::Ride.string());
    assert_eq!("Snowboard", ActivityType::Snowboard.string());
}

#[test]
fn resource_state_values() {
    assert_eq!(ResourceState::Meta as i32, 1);
    assert_eq!(ResourceState::Summary as i32, 2);
    assert_eq!(ResourceState::Detailed as i32, 3);
}

#[test]
fn default_resource_state() {
    let default_state: ResourceState = Default::default();
    assert_eq!(default_state, ResourceState::Unknown);
}

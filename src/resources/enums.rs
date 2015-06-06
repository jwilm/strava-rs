use std::default::Default;

use rustc_serialize::{Decodable, Decoder};

/// Objects will be returned with a certain ResourceState
///
/// Detailed contains the most data and Meta the least.
#[allow(dead_code)]
#[derive(Debug, PartialEq, RustcEncodable)]
pub enum ResourceState {
    Unknown,
    Meta,
    Summary,
    Detailed
}

// TODO refactor primitive conversion into custom trait. Maybe add a macro or compiler plugin to
// handle this.
impl Decodable for ResourceState {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let num = try!(d.read_u8());
        Ok(match num {
            0 => ResourceState::Unknown,
            1 => ResourceState::Meta,
            2 => ResourceState::Summary,
            3 => ResourceState::Detailed,
            _ =>  unreachable!("ResourceState only valid for 0,1,2,3")
        })
    }
}

impl Default for ResourceState {
    fn default () -> ResourceState { ResourceState::Unknown }
}

/// Frame type for bikes
#[allow(dead_code)]
#[derive(Debug)]
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

pub enum ValueError {
    InvalidValue,
    Other
}

/// Types of clubs
#[allow(dead_code)]
#[derive(Debug)]
pub enum ClubType {
    Casual,
    Racing,
    Triathlon,
    Other
}

/// Types of sports
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportType {
    Cycling,
    Running,
    Triathlon,
    Other
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum WorkoutType {
    Default,
    Race,
    Long,
    Intervals
}

#[cfg(test)]
mod tests {
    use std::default::Default;

    use super::ResourceState;

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
}

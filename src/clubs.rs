//! **UNIMPLEMENTED:** Groups of athletes on Strava
use api::ResourceState;

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

/// Types of sports
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportType {
    Cycling,
    Running,
    Triathlon,
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

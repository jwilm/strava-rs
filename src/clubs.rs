use api::ResourceState;

/// Clubs represent groups of athletes on Strava.
///
/// They can be public or private. Only members of private clubs can access
/// their details. The object is returned in summary or detailed
/// representations.
///
/// See: http://strava.github.io/api/v3/clubs/
#[allow(dead_code)]
#[derive(Debug, RustcDecodable)]
pub struct Club {
    id: i32,
    resource_state: ResourceState,
    name: String,
    profile_medium: String,
    profile: String,
    cover_photo: String,
    cover_photo_small: String,
    sport_type: SportType,
    city: String,
    state: String,
    country: String,
    private: bool,
    member_count: i32,
    featured: bool,
    verified: bool,
    url: String,

    description: Option<String>,
    club_type: Option<ClubType>, 
    membership: Option<String>,
    admin: Option<bool>,
    owner: Option<bool>,
    following_count: Option<i32>,  
}

/// Types of sports
#[derive(Debug, RustcDecodable)]
pub enum SportType {
    Cycling,
    Running,
    Triathlon,
    Other
}

/// Types of clubs
#[derive(Debug, RustcDecodable)]
pub enum ClubType {
    Casual,
    Racing,
    Triathlon,
    Other
}

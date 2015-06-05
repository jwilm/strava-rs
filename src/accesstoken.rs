use std::env;
use std::convert::From;

pub struct AccessToken {
    token: String
}

impl AccessToken {
    pub fn new(token: String) -> AccessToken {
        AccessToken {
            token: token
        }
    }

    pub fn new_from_env() -> Result<AccessToken, env::VarError> {
        match env::var("STRAVA_ACCESS_TOKEN") {
            Ok(token) => Ok(AccessToken::new(token)),
            Err(e) => Err(e)
        }
    }
}

impl<'a> From<&'a str> for AccessToken {
    fn from(s: &'a str) -> AccessToken {
        AccessToken { token: s.to_string() }
    }
}

impl AccessToken {
    pub fn get(&self) -> &str {
        &self.token[..]
    }
}

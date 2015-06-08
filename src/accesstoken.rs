use std::env;
use std::convert::From;

/// A strava.com api access token.
///
/// You'll need to register/login at https://www.strava.com/developers to get a token. This is
/// required for all requests.
pub struct AccessToken {
    token: String
}

impl AccessToken {
    /// Create an AccessToken from the supplied string
    pub fn new(token: String) -> AccessToken {
        AccessToken {
            token: token
        }
    }

    /// Create an AccessToken from the environment variable STRAVA_ACCESS_TOKEN
    pub fn new_from_env() -> Result<AccessToken, env::VarError> {
        match env::var("STRAVA_ACCESS_TOKEN") {
            Ok(token) => Ok(AccessToken::new(token)),
            Err(e) => Err(e)
        }
    }

    /// Get the token underlying string
    ///
    /// This is used internally for building requests.
    // TODO implement Deref -> &str for AccessToken
    pub fn get(&self) -> &str {
        &self.token[..]
    }
}

impl<'a> From<&'a str> for AccessToken {
    fn from(s: &'a str) -> AccessToken {
        AccessToken { token: s.to_string() }
    }
}

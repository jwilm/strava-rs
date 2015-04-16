use std::convert::From;

pub struct AccessToken {
    token: &'static str
}

impl From<&'static str> for AccessToken {
    fn from(s: &'static str) -> AccessToken {
        AccessToken { token: s }
    }
}

impl AccessToken {
    pub fn get(&self) -> &'static str {
        self.token
    }
}

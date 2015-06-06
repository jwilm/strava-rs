use accesstoken::AccessToken;

pub fn v3(token: &AccessToken, url: String) -> String {
    format!("https://www.strava.com/api/v3/{}?access_token={}", url, token.get())
}

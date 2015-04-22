strava-rs
=========

Strava API client written in rust

## About

This library is highly incomplete. At this point in time, you are able to fetch
the athlete associated with an auth token. Docs will be arriving soon.

```rust
extern crate strava;

use strava::AccessToken;
use strava::Athlete;

fn main() {
    let token = AccessToken::from("<replace me>");
    let athlete = Athlete::get_current(&token).unwrap();
    println!("{:?}", athlete);
}
```

## Disclaimer

I am not in any way affiliated with Strava, Inc. I merely wish to use the Strava
API from Rust.

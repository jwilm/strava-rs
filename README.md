strava-rs
=========

Strava API client in Rust

[![Circle CI](https://circleci.com/gh/jwilm/strava-rs.svg?style=svg)](https://circleci.com/gh/jwilm/strava-rs)

## About

The library currently exposes functions for accessing athletes, segments, and
segment efforts. Support for activities is next on the list. For an exhaustive
list of capabilities, please reference the [docs][].

```rust
extern crate strava;

use strava::athletes::Athlete;
use strava::api::AccessToken;

fn main() {
    // Create a token
    let token = AccessToken::new("<my token>".to_string());

    // Get the athlete associated with the given token
    let athlete = Athlete::get_current(&token).unwrap();

    // All of the strava types implement Debug and can be printed like so:
    println!("{:?}", athlete);
}
```

## Disclaimer

I am not in any way affiliated with Strava, Inc. I merely wish to use the Strava
API from Rust.

[docs]: http://www.jwilm.io/strava-rs/strava/

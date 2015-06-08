use accesstoken::AccessToken;

#[doc(hidden)]
pub fn v3(token: &AccessToken, url: String) -> String {
    format!("https://www.strava.com/api/v3/{}?access_token={}", url, token.get())
}

/// Wrapper for endpoints that paginate
///
/// A Paginated<T> will be returned from any endpoint that supports paging. Provides methods for
/// fetching the next page and checking if more pages are available.
#[derive(Debug)]
pub struct Paginated<T> {
    page: usize,
    per_page: usize,
    url: String,
    data: Vec<T>
}

impl<T> Paginated<T> {
    pub fn new(url: String, data: Vec<T>) -> Paginated<T> {
        Paginated {
            page: 1,
            per_page: 30,
            url: url,
            data: data,
        }
    }

    /// Get the next page of results
    pub fn next(&self) -> Option<Paginated<T>> {
        unimplemented!();
    }

    /// Check if this is the last page
    pub fn last_page(&self) -> bool {
        self.per_page != self.data.len()
    }
}

#[cfg(test)]
mod paginated_tests {
    use super::Paginated;

    #[test]
    fn last_page() {
        let vec = (0..30).collect::<Vec<u8>>();
        let pager = Paginated::new("test".to_string(), vec);
        println!("{:?}", pager);
        assert_eq!(pager.last_page(), false);
    }
}

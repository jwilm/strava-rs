#[derive(Debug)]
pub struct Paginated<T> {
    page: i32,
    per_page: i32,
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
}

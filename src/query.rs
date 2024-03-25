struct Query {
    url: String,
    params: std::vec::Vec<String>,
}

impl Query {
    fn new(url: String) -> Self {
        Self {
            url,
            params: vec![],
        }
    }
}

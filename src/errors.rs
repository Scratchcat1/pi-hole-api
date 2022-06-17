#[derive(Debug)]
pub enum APIError {
    RequestError(reqwest::Error),
    MissingAPIKey,
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> Self {
        APIError::RequestError(error)
    }
}

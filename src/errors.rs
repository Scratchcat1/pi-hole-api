#[derive(Debug)]
pub enum APIError {
    RequestError(reqwest::Error),
    SerdeJSONError(serde_json::Error),
    MissingAPIKey,
    InvalidList,
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> Self {
        APIError::RequestError(error)
    }
}

impl From<serde_json::Error> for APIError {
    fn from(error: serde_json::Error) -> Self {
        APIError::SerdeJSONError(error)
    }
}

use crate::api_types::FTLNotRunning;

#[derive(Debug)]
pub enum APIError {
    RequestError(reqwest::Error),
    SerdeJSONError(serde_json::Error),
    MissingAPIKey,
    InvalidList,
    FTLNotRunning,
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

/// Filter out response errors from the API
pub fn detect_response_errors(response_text: &str) -> Result<(), APIError> {
    if response_text.starts_with("Invalid list") {
        return Err(APIError::InvalidList);
    }
    if let Ok(ftl_response) = serde_json::from_str::<FTLNotRunning>(response_text) {
        if !ftl_response.ftl_not_running {
            assert!(!ftl_response.ftl_not_running);
            return Err(APIError::FTLNotRunning);
        }
    }

    Ok(())
}

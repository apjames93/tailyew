use gloo_net::http::{Method, Request};
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::helpers::get_token;

pub enum ResponseError {
    StatusCodeError(u16, String),
    SerializationError(String),
    NetworkError(String),
}

// Implement Display for ResponseError to improve error messages
impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::StatusCodeError(code, message) => {
                write!(f, "HTTP {}: {}", code, message)
            }
            ResponseError::SerializationError(message) => {
                write!(f, "Serialization error: {}", message)
            }
            ResponseError::NetworkError(message) => write!(f, "Network error: {}", message),
        }
    }
}


/// Sends an HTTP request with the specified method and request body to the provided URL.
/// Handles serialization, deserialization, and error responses.
pub async fn send_request<T, U>(
    request_url: String,
    method: Method,
    req: Option<T>,
) -> Result<U, ResponseError>
where
    T: Serialize,
    U: for<'de> Deserialize<'de>,
{
    web_sys::console::log_1(&format!("Initiating request to: {}", request_url).into());

    let mut request_builder = match method {
        Method::POST => Request::post(&request_url),
        Method::GET => Request::get(&request_url),
        Method::PUT => Request::put(&request_url),
        Method::DELETE => Request::delete(&request_url),
        _ => {
            return Err(ResponseError::NetworkError(
                "Unsupported HTTP method".to_string(),
            ))
        }
    };

    // Set headers conditionally
    if req.is_some() {
        request_builder = request_builder.header("Content-Type", "application/json");
    }

    if let Some(token) = get_token() {
        request_builder = request_builder.header("Authorization", &format!("Bearer {}", token));
    }

    // Build request with body if provided
    let request = if let Some(body) = req {
        let serialized_body = serde_json::to_string(&body).map_err(|e| {
            ResponseError::SerializationError(format!("Failed to serialize request: {}", e))
        })?;
        request_builder
            .body(serialized_body)
            .map_err(|e| ResponseError::NetworkError(format!("Failed to build request: {}", e)))?
    } else {
        request_builder
            .build()
            .map_err(|e| ResponseError::NetworkError(format!("Failed to build request: {}", e)))?
    };

    // Send request and handle response
    let response = request
        .send()
        .await
        .map_err(|e| ResponseError::NetworkError(format!("Failed to send request: {}", e)))?;

    // Handle non-OK responses
    if !response.ok() {
        let status_code = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error occurred".to_string());

        return Err(ResponseError::StatusCodeError(status_code, error_text));
    }

    // Parse response text and deserialize
    let response_text = response
        .text()
        .await
        .map_err(|e| ResponseError::NetworkError(format!("Failed to read response: {}", e)))?;

    serde_json::from_str(&response_text).map_err(|e| {
        ResponseError::SerializationError(format!("Failed to deserialize response: {}", e))
    })
}

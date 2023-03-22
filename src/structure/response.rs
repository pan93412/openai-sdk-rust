//! The structure of a OpenAI response.

use serde::Deserialize;
use serde_json::Value;

/// The response of OpenAI.
///
/// It has two cases:
///   - Ok(T) - the response is successful
///   - Error(OuterErrorResponse) - the response is failed
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Ok(T),
    Error(OuterErrorResponse),
}

/// The outer part of the error response.
#[derive(Deserialize, Debug)]
pub struct OuterErrorResponse {
    pub error: ErrorResponse,
}

/// The error response (code, message, type, etc.)
// TODO)) i'm not pretty sure about this structure
#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: Option<Value>,
    pub message: Option<String>, // Invalid URL (GET /v1/model/text-babbage:001)
    pub param: Option<Value>,
    #[serde(rename = "type")]
    pub type_: Option<String>, // invalid_request_error
}

/// The response result.
pub type RespResult<T, E> = Result<Response<T>, E>;

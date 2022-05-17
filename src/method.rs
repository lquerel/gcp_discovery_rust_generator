//! An individual method description.
use serde_with::skip_serializing_none;

use crate::parameter::Parameter;
use std::collections::HashMap;
use crate::media_upload::MediaUpload;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    ///A unique ID for this method. This property can be used to match methods between different versions of Discovery.
    pub id: String,
    /// The URI path of this REST method. Should be used in conjunction with the servicePath property at the API-level.
    pub path: String,
    /// The URI path of this REST method in (RFC 6570) format without level 2 features ({+var}). Supplementary to the path property.
    pub flat_path: String,
    /// HTTP method used by this method.
    pub http_method: String,
    ///Description of this method.
    pub description: String,
    /// Details for all parameters in this method.
    pub parameters: HashMap<String,Parameter>,
    /// Ordered list of required parameters. This serves as a hint to clients on how to structure their method signatures. The array is ordered such that the most significant parameter appears first.
    pub parameter_order: Option<Vec<String>>,
    /// OAuth 2.0 scopes applicable to this method.
    pub scopes: Vec<String>,
    /// Whether this method supports media downloads.
    pub supports_media_download: Option<bool>,
    /// Whether this method supports media uploads.
    pub supports_media_upload: Option<bool>,
    /// Media upload parameters.
    pub media_upload: Option<MediaUpload>,
    /// Whether this method supports subscriptions.
    pub supports_subscription: Option<bool>,
    /// The schema for the request.
    pub request: Option<Request>,
    /// The schema for the response.
    pub response: Option<Response>,
}

/// The schema for the request.
#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    ///Schema ID for the request schema.
    #[serde(rename = "$ref")]
    pub r#ref: String,

}

/// The schema for the response.
#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    ///Schema ID for the response schema.
    #[serde(rename = "$ref")]
    pub r#ref: String,
}
//! Media upload parameters.
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaUpload {
    /// MIME Media Ranges for acceptable media uploads to this method.
    pub accept: Vec<String>,
    /// Maximum size of a media upload, such as "1MB", "2GB" or "3TB".
    pub max_size: String,
    /// Supported upload protocols.
    pub protocols: Protocols,
}

/// Supported upload protocols.
#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocols {
    /// Supports uploading as a single HTTP request.
    pub simple: Option<Simple>,
    /// Supports the Resumable Media Upload protocol.
    pub resumable: Option<Resumable>,
}

/// Supports uploading as a single HTTP request.
#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Simple {
    /// true if this endpoint supports upload multipart media.
    pub multipart: bool,
    ///The URI path to be used for upload. Should be used in conjunction with the rootURL property at the API-level.
    pub path: String,
}

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resumable {
    /// true if this endpoint supports uploading multipart media.
    pub multipart: bool,
    ///The URI path to be used for upload. Should be used in conjunction with the rootURL property at the API-level.
    pub path: String,
}
//! Links to 16x16 and 32x32 icons representing the API.
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Icons {
    /// The URL of the 16x16 icon.
    pub x16: String,
    /// The URL of the 32x32 icon.
    pub x32: String,
}
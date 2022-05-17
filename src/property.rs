//!
use serde_with::skip_serializing_none;

use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub r#type: Option<String>,
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub format: Option<String>,
    pub description: Option<String>,
    pub items: Option<HashMap<String, serde_json::Value>>,
    pub r#enum: Option<Vec<String>>,
    pub enum_descriptions: Option<Vec<String>>,
    pub additional_properties: Option<HashMap<String, serde_json::Value>>
}

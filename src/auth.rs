//! Authentication information.

use std::collections::HashMap;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    /// OAuth 2.0 authentication information.
    pub oauth2: OAuth2,
}

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2 {
    /// Available OAuth 2.0 scopes.
    pub scopes: HashMap<String, Scope>,
}

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    /// Description of scope.
    pub description: String,
}
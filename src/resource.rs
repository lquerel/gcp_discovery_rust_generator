//! An individual resource description. Contains methods and sub-resources related to this resource.
use serde_with::skip_serializing_none;

use crate::method::Method;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Methods on this resource.
    pub methods: Option<HashMap<String, Method>>,
    /// Sub-resources on this resource.
    pub resources: Option<HashMap<String, Resource>>,
}

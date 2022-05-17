use crate::icons::Icons;
use crate::parameter::Parameter;
use std::collections::HashMap;
use crate::auth::Auth;
use crate::schema::Schema;
use crate::method::Method;
use crate::resource::Resource;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryDocument {
    /// The kind for this response. The fixed string discovery#restDescription.
    pub kind: String,
    /// Indicate the version of the Discovery API used to generate this doc.
    pub discovery_version: Option<String>,
    /// The ID of the Discovery document for the API. For example, urlshortener:v1.
    pub id: Option<String>,
    /// The name of the API. For example, urlshortener.
    pub name: String,
    /// The canonical name of the API. For example, Url Shortener.
    pub canonical_name: Option<String>,
    /// The version of the API. For example, v1.
    pub version: String,
    /// The revision of the API.
    pub revision: String,
    /// The title of the API. For example, "Google Url Shortener API".
    pub title: Option<String>,
    /// The description of this API.
    pub description: Option<String>,
    /// Links to 16x16 and 32x32 icons representing the API.
    pub icons: Icons,
    /// A link to human-readable documentation for the API.
    pub documentation_link: Option<String>,
    /// Labels for the status of this API. Valid values include limited_availability or deprecated.
    pub labels: Option<Vec<String>>,
    /// The protocol described by the document. For example, REST.
    pub protocol: String,
    /// The root url under which all API services live.
    pub root_url: Option<String>,
    /// Common parameters that apply across all apis.
    pub parameters: HashMap<String,Parameter>,
    /// Authentication information.
    pub auth: Auth,
    /// A list of supported features for this API.
    pub features: Option<Vec<String>>,
    /// The schemas for this API.
    pub schemas: HashMap<String, Schema>,
    /// API-level methods for this API.
    pub methods: Option<HashMap<String, Method>>,
    /// The base path for all REST requests.
    pub service_path: Option<String>,
    /// The path for REST batch requests.
    pub batch_path: Option<String>,
    /// The resources in this API.
    pub resources: Option<HashMap<String, Resource>>,
    /// [DEPRECATED] The base URL for REST requests.
    pub base_path: Option<String>,
    /// [DEPRECATED] The base path for REST requests.
    pub base_url: Option<String>,
    pub fully_encode_reserved_expansion: Option<bool>,
    pub mtls_root_url: Option<String>,
    pub owner_domain: Option<String>,
    pub owner_name: Option<String>,
    #[serde(rename = "version_module")]
    pub version_module: Option<bool>,
}
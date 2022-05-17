//! Description of a single parameter.
use serde_with::skip_serializing_none;

use std::collections::HashMap;
use crate::property::Property;

#[skip_serializing_none]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// Unique identifier for this schema.
    pub id: Option<String>,
    /// The value type for this schema. A list of values can be found at the "type" section in the JSON Schema.
    pub r#type: Option<String>,
    /// A reference to another schema. The value of this property is the ID of another schema.
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    /// A description of this object.
    pub description: String,
    /// The default value of this property (if one exists).
    pub default: Option<String>,
    /// Whether the parameter is required.
    pub required: Option<bool>,
    /// An additional regular expression or key that helps constrain the value. For more details see the Type and Format Summary.
    pub format: Option<String>,
    /// The regular expression this parameter must conform to.
    pub pattern: Option<String>,
    /// The minimum value of this parameter.
    pub minimum: Option<String>,
    /// The maximum value of this parameter.
    pub maximum: Option<String>,
    /// Values this parameter may take (if it is an enum).
    pub r#enum: Option<Vec<String>>,
    /// The descriptions for the enums. Each position maps to the corresponding value in the enum array.
    pub enum_descriptions: Option<Vec<String>>,
    /// Whether this parameter may appear multiple times.
    pub repeated: Option<bool>,
    /// Whether this parameter goes in the query or the path for REST requests.
    pub location: Option<String>,
    /// If this is a schema for an object, list the schema for each property of this object.
    pub properties: Option<HashMap<String,Property>>,
    /// If this is a schema for an object, this property is the schema for any additional properties with dynamic keys on this object.
    pub additional_properties: Option<String>,
    /// If this is a schema for an array, this property is the schema for each element in the array.
    pub items: Option<String>,
    /// Additional information about this property.
    pub annotations: Option<String>,
}

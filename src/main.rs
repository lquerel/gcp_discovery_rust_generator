#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_with;

use crate::discovery_doc::DiscoveryDocument;
use crate::property::Property;
use inflector::Inflector;
use std::collections::HashSet;

mod discovery_doc;
mod icons;
mod parameter;
mod auth;
mod schema;
mod property;
mod method;
mod media_upload;
mod resource;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loaded_json = reqwest::get("https://monitoring.googleapis.com/$discovery/rest?version=v3")
        .await?
        .text()
        .await?;

    let result: Result<DiscoveryDocument, serde_json::error::Error> = serde_json::from_str(&loaded_json);
    match result {
        Err(err) => {
            dbg!(&err);
        }
        Ok(discovery_doc) => {
            for schema in discovery_doc.schemas.keys() {
                let gen_code = generate_struct("", schema, &discovery_doc);
                println!("{}\n", gen_code);
            }
        }
    }

    // Used http://www.jsondiff.com/ to compare json docs

    Ok(())
}

fn generate_struct(indent: &str, schema_name: &str, discovery_doc: &DiscoveryDocument) -> String {
    let mut structs_to_import = HashSet::new();
    let mut struct_code = String::new();

    if let Some(schema) = discovery_doc.schemas.get(schema_name) {
        if let Some(description) = schema.description.as_ref() {
            struct_code.push_str(&format!("/// {}", description));
        }
        struct_code.push_str(&format!(r##"
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {} {{
"##, schema_name.to_class_case()));
        if let Some(properties) = &schema.properties {
            for (property_name, property) in properties {
                struct_code.push_str(&gen_comment("\t", property));
                struct_code.push_str(&gen_attribute("\t", property_name, property, &mut structs_to_import));
            }
        }
        struct_code.push_str("}\n");
    }

    if structs_to_import.is_empty() {
        struct_code
    } else {
        let use_section = structs_to_import.iter().map(|s| format!("use crate::model::{}::{};", rust_module(s), s)).collect::<Vec<String>>().join("\n");
        format!("{}\n\n{}", use_section, struct_code)
    }
}

fn gen_attribute(indent: &str, property_name: &str, property: &Property, structs_to_import: &mut HashSet<String>) -> String {
    let mut code = String::new();
    let property_name = rust_attribute(property_name);

    if let Some(r#ref) = &property.r#ref {
        let struct_id = rust_struct(r#ref, structs_to_import);
        code.push_str(&format!("{}pub {} Option<{}>,\n", indent, property_name, struct_id));
    }

    if let Some(r#type) = &property.r#type {
        match r#type.as_ref() {
            "array" => {
                let item_type = property.items
                    .as_ref()
                    .expect(&format!("Invalid property definition, the field 'items' must be defined for a property of type array"))
                    .get("$ref")
                    .expect("Invalid property definition, the field '$ref' must be defined in 'items' for a property of type array")
                    .as_str()
                    .expect("Invalid property definition, the field '$ref' must be a string in 'items' for a property of type array");
                code.push_str(&format!("{}pub {} Option<Vec<{}>>,\n", indent, property_name, rust_type(&item_type, structs_to_import)));
            }
            "object" => {
                let item_type = property.additional_properties
                    .as_ref()
                    .expect("Invalid property definition, the field 'additionalProperties' must be defined for a property of type object")
                    .get("type")
                    .expect("Invalid property definition, the field 'type' must be defined in 'additionalProperties' for a property of type object")
                    .as_str()
                    .expect("Invalid property definition, the field 'type' must be a string in 'additionalProperties' for property of type object");
                code.push_str(&format!("{}pub {} Option<HashMap<String, {}>>,\n", indent, property_name, rust_type(item_type, structs_to_import)));
            }
            _ => {
                code.push_str(&format!("{}pub {} Option<{}>,\n", indent, property_name, rust_type(r#type, structs_to_import)));
            }
        }
    }

    code
}

fn gen_comment(indent: &str, property: &Property) -> String {
    let mut code = String::new();

    if let Some(description) = &property.description {
        code.push_str(&format!("{}/// {}\n", indent, description));
    }

    code
}

fn rust_module(identifier: &str) -> String {
    identifier.to_snake_case()
}

fn rust_struct(identifier: &str, structs_to_import: &mut HashSet<String>) -> String {
    let struct_id = identifier.to_class_case();
    structs_to_import.insert(struct_id.clone());
    struct_id
}

fn rust_attribute(identifier: &str) -> String {
    identifier.to_snake_case()
}

fn rust_type(api_type: &str, structs_to_import: &mut HashSet<String>) -> String {
    match api_type {
        "string" => "String".into(),
        "boolean" => "bool".into(),
        _ => {
            rust_struct(api_type, structs_to_import)
        },
    }
}
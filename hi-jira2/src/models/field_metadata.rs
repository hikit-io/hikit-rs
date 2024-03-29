/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldMetadata : The metadata describing an issue field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldMetadata {
    /// Whether the field is required.
    #[serde(rename = "required")]
    pub required: bool,
    /// The data type of the field.
    #[serde(rename = "schema")]
    pub schema: Option<crate::models::JsonTypeBean>,
    /// The name of the field.
    #[serde(rename = "name")]
    pub name: String,
    /// The key of the field.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// The URL that can be used to automatically complete the field.
    #[serde(rename = "autoCompleteUrl", skip_serializing_if = "Option::is_none")]
    pub auto_complete_url: Option<String>,
    /// Whether the field has a default value.
    #[serde(rename = "hasDefaultValue", skip_serializing_if = "Option::is_none")]
    pub has_default_value: Option<bool>,
    /// The list of operations that can be performed on the field.
    #[serde(rename = "operations")]
    pub operations: Vec<String>,
    /// The list of values allowed in the field.
    #[serde(rename = "allowedValues", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<serde_json::Value>>,
    /// The default value of the field.
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    /// The configuration properties.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl FieldMetadata {
    /// The metadata describing an issue field.
    pub fn new(required: bool, schema: Option<crate::models::JsonTypeBean>, name: String, operations: Vec<String>) -> FieldMetadata {
        FieldMetadata {
            required,
            schema: schema,
            name,
            key:None,
            auto_complete_url: None,
            has_default_value: None,
            operations,
            allowed_values: None,
            default_value: None,
            configuration: None,
        }
    }
}



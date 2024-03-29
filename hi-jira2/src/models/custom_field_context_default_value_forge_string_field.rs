/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueForgeStringField : The default text for a Forge string custom field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeStringField {
    /// The default text. The maximum length is 254 characters.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CustomFieldContextDefaultValueForgeStringField {
    /// The default text for a Forge string custom field.
    pub fn new() -> CustomFieldContextDefaultValueForgeStringField {
        CustomFieldContextDefaultValueForgeStringField {
            text: None,
        }
    }
}



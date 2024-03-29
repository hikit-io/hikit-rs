/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueUpdate : Default values to update.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueUpdate {
    #[serde(rename = "defaultValues", skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<crate::models::CustomFieldContextDefaultValue>>,
}

impl CustomFieldContextDefaultValueUpdate {
    /// Default values to update.
    pub fn new() -> CustomFieldContextDefaultValueUpdate {
        CustomFieldContextDefaultValueUpdate {
            default_values: None,
        }
    }
}



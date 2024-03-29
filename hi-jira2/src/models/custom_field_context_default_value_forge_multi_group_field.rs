/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueForgeMultiGroupField : The default value for a Forge collection of groups custom field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeMultiGroupField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The IDs of the default groups.
    #[serde(rename = "groupIds")]
    pub group_ids: Vec<String>,
}

impl CustomFieldContextDefaultValueForgeMultiGroupField {
    /// The default value for a Forge collection of groups custom field.
    pub fn new(context_id: String, group_ids: Vec<String>) -> CustomFieldContextDefaultValueForgeMultiGroupField {
        CustomFieldContextDefaultValueForgeMultiGroupField {
            context_id,
            group_ids,
        }
    }
}



/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldUpdatedContextOptionsList : A list of custom field options for a context.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldUpdatedContextOptionsList {
    /// The updated custom field options.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::CustomFieldOptionUpdate>>,
}

impl CustomFieldUpdatedContextOptionsList {
    /// A list of custom field options for a context.
    pub fn new() -> CustomFieldUpdatedContextOptionsList {
        CustomFieldUpdatedContextOptionsList {
            options: None,
        }
    }
}


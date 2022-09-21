/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PropertyKeys : List of property keys.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PropertyKeys {
    /// Property key details.
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<crate::models::PropertyKey>>,
}

impl PropertyKeys {
    /// List of property keys.
    pub fn new() -> PropertyKeys {
        PropertyKeys {
            keys: None,
        }
    }
}


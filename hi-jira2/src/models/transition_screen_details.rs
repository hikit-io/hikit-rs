/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// TransitionScreenDetails : The details of a transition screen.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransitionScreenDetails {
    /// The ID of the screen.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the screen.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TransitionScreenDetails {
    /// The details of a transition screen.
    pub fn new(id: String) -> TransitionScreenDetails {
        TransitionScreenDetails {
            id,
            name: None,
        }
    }
}


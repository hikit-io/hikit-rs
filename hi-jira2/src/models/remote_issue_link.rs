/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RemoteIssueLink : Details of an issue remote link.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteIssueLink {
    /// The ID of the link.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The URL of the link.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The global ID of the link, such as the ID of the item on the remote system.
    #[serde(rename = "globalId", skip_serializing_if = "Option::is_none")]
    pub global_id: Option<String>,
    /// Details of the remote application the linked item is in.
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::models::Application>,
    /// Description of the relationship between the issue and the linked item.
    #[serde(rename = "relationship", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// Details of the item linked to.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::models::RemoteObject>,
}

impl RemoteIssueLink {
    /// Details of an issue remote link.
    pub fn new() -> RemoteIssueLink {
        RemoteIssueLink {
            id: None,
            _self: None,
            global_id: None,
            application: None,
            relationship: None,
            object: None,
        }
    }
}



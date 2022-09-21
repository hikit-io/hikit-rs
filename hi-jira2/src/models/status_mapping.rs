/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// StatusMapping : Details about the mapping from a status to a new status for an issue type.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusMapping {
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the status.
    #[serde(rename = "statusId")]
    pub status_id: String,
    /// The ID of the new status.
    #[serde(rename = "newStatusId")]
    pub new_status_id: String,
}

impl StatusMapping {
    /// Details about the mapping from a status to a new status for an issue type.
    pub fn new(issue_type_id: String, status_id: String, new_status_id: String) -> StatusMapping {
        StatusMapping {
            issue_type_id,
            status_id,
            new_status_id,
        }
    }
}


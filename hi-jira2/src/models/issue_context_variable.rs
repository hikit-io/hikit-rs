/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueContextVariable : An [issue](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#issue) specified by ID or key. All the fields of the issue object are available in the Jira expression.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueContextVariable {
    /// Type of custom context variable.
    #[serde(rename = "type")]
    pub _type: String,
    /// The issue ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The issue key.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl IssueContextVariable {
    /// An [issue](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#issue) specified by ID or key. All the fields of the issue object are available in the Jira expression.
    pub fn new(_type: String) -> IssueContextVariable {
        IssueContextVariable {
            _type,
            id: None,
            key: None,
        }
    }
}



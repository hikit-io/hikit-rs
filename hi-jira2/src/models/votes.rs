/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Votes : The details of votes on an issue.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Votes {
    /// The URL of these issue vote details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The number of votes on the issue.
    #[serde(rename = "votes", skip_serializing_if = "Option::is_none")]
    pub votes: Option<i64>,
    /// Whether the user making this request has voted on the issue.
    #[serde(rename = "hasVoted", skip_serializing_if = "Option::is_none")]
    pub has_voted: Option<bool>,
    /// List of the users who have voted on this issue. An empty list is returned when the calling user doesn't have the *View voters and watchers* project permission.
    #[serde(rename = "voters", skip_serializing_if = "Option::is_none")]
    pub voters: Option<Vec<crate::models::User>>,
}

impl Votes {
    /// The details of votes on an issue.
    pub fn new() -> Votes {
        Votes {
            _self: None,
            votes: None,
            has_voted: None,
            voters: None,
        }
    }
}



/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueChangelogIds : A list of changelog IDs.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueChangelogIds {
    /// The list of changelog IDs.
    #[serde(rename = "changelogIds")]
    pub changelog_ids: Vec<i64>,
}

impl IssueChangelogIds {
    /// A list of changelog IDs.
    pub fn new(changelog_ids: Vec<i64>) -> IssueChangelogIds {
        IssueChangelogIds {
            changelog_ids,
        }
    }
}



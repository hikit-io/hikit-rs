/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WebhookDetails : A list of webhooks.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebhookDetails {
    /// The JQL filter that specifies which issues the webhook is sent for. Only a subset of JQL can be used. The supported elements are:   *  Fields: `issueKey`, `project`, `issuetype`, `status`, `assignee`, `reporter`, `issue.property`, and `cf[id]`. For custom fields (`cf[id]`), only the epic label custom field is supported.\".  *  Operators: `=`, `!=`, `IN`, and `NOT IN`.
    #[serde(rename = "jqlFilter")]
    pub jql_filter: String,
    /// A list of field IDs. When the issue changelog contains any of the fields, the webhook `jira:issue_updated` is sent. If this parameter is not present, the app is notified about all field updates.
    #[serde(rename = "fieldIdsFilter", skip_serializing_if = "Option::is_none")]
    pub field_ids_filter: Option<Vec<String>>,
    /// A list of issue property keys. A change of those issue properties triggers the `issue_property_set` or `issue_property_deleted` webhooks. If this parameter is not present, the app is notified about all issue property updates.
    #[serde(rename = "issuePropertyKeysFilter", skip_serializing_if = "Option::is_none")]
    pub issue_property_keys_filter: Option<Vec<String>>,
    /// The Jira events that trigger the webhook.
    #[serde(rename = "events")]
    pub events: Vec<Events>,
}

impl WebhookDetails {
    /// A list of webhooks.
    pub fn new(jql_filter: String, events: Vec<Events>) -> WebhookDetails {
        WebhookDetails {
            jql_filter,
            field_ids_filter: None,
            issue_property_keys_filter: None,
            events,
        }
    }
}

/// The Jira events that trigger the webhook.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "jira:issue_created")]
    JiraissueCreated,
    #[serde(rename = "jira:issue_updated")]
    JiraissueUpdated,
    #[serde(rename = "jira:issue_deleted")]
    JiraissueDeleted,
    #[serde(rename = "comment_created")]
    CommentCreated,
    #[serde(rename = "comment_updated")]
    CommentUpdated,
    #[serde(rename = "comment_deleted")]
    CommentDeleted,
    #[serde(rename = "issue_property_set")]
    IssuePropertySet,
    #[serde(rename = "issue_property_deleted")]
    IssuePropertyDeleted,
}

impl Default for Events {
    fn default() -> Events {
        Self::JiraissueCreated
    }
}


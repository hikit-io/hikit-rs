/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Comment : A comment.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Comment {
    /// The URL of the comment.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The ID of the comment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the user who created the comment.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<crate::models::UserDetails>,
    /// The comment text.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The rendered version of the comment.
    #[serde(rename = "renderedBody", skip_serializing_if = "Option::is_none")]
    pub rendered_body: Option<String>,
    /// The ID of the user who updated the comment last.
    #[serde(rename = "updateAuthor", skip_serializing_if = "Option::is_none")]
    pub update_author: Option<crate::models::UserDetails>,
    /// The date and time at which the comment was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The date and time at which the comment was updated last.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// The group or role to which this comment is visible. Optional on create and update.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<crate::models::Visibility>,
    /// Whether the comment is visible in Jira Service Desk. Defaults to true when comments are created in the Jira Cloud Platform. This includes when the site doesn't use Jira Service Desk or the project isn't a Jira Service Desk project and, therefore, there is no Jira Service Desk for the issue to be visible on. To create a comment with its visibility in Jira Service Desk set to false, use the Jira Service Desk REST API [Create request comment](https://developer.atlassian.com/cloud/jira/service-desk/rest/#api-rest-servicedeskapi-request-issueIdOrKey-comment-post) operation.
    #[serde(rename = "jsdPublic", skip_serializing_if = "Option::is_none")]
    pub jsd_public: Option<bool>,
    /// Whether the comment was added from an email sent by a person who is not part of the issue. See [Allow external emails to be added as comments on issues](https://support.atlassian.com/jira-service-management-cloud/docs/allow-external-emails-to-be-added-as-comments-on-issues/)for information on setting up this feature.
    #[serde(rename = "jsdAuthorCanSeeRequest", skip_serializing_if = "Option::is_none")]
    pub jsd_author_can_see_request: Option<bool>,
    /// A list of comment properties. Optional on create and update.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::EntityProperty>>,
}

impl Comment {
    /// A comment.
    pub fn new() -> Comment {
        Comment {
            _self: None,
            id: None,
            author: None,
            body: None,
            rendered_body: None,
            update_author: None,
            created: None,
            updated: None,
            visibility: None,
            jsd_public: None,
            jsd_author_can_see_request: None,
            properties: None,
        }
    }
}



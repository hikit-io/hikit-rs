/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionEvalContextBean {
    /// The issue that is available under the `issue` variable when evaluating the expression.
    #[serde(rename = "issue", skip_serializing_if = "Option::is_none")]
    pub issue: Option<crate::models::IdOrKeyBean>,
    /// The collection of issues that is available under the `issues` variable when evaluating the expression.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<crate::models::JexpIssues>,
    /// The project that is available under the `project` variable when evaluating the expression.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::IdOrKeyBean>,
    /// The ID of the sprint that is available under the `sprint` variable when evaluating the expression.
    #[serde(rename = "sprint", skip_serializing_if = "Option::is_none")]
    pub sprint: Option<i64>,
    /// The ID of the board that is available under the `board` variable when evaluating the expression.
    #[serde(rename = "board", skip_serializing_if = "Option::is_none")]
    pub board: Option<i64>,
    /// The ID of the service desk that is available under the `serviceDesk` variable when evaluating the expression.
    #[serde(rename = "serviceDesk", skip_serializing_if = "Option::is_none")]
    pub service_desk: Option<i64>,
    /// The ID of the customer request that is available under the `customerRequest` variable when evaluating the expression. This is the same as the ID of the underlying Jira issue, but the customer request context variable will have a different type.
    #[serde(rename = "customerRequest", skip_serializing_if = "Option::is_none")]
    pub customer_request: Option<i64>,
    /// Custom context variables and their types. These variable types are available for use in a custom context:   *  `user`: A [user](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#user) specified as an Atlassian account ID.  *  `issue`: An [issue](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#issue) specified by ID or key. All the fields of the issue object are available in the Jira expression.  *  `json`: A JSON object containing custom content.  *  `list`: A JSON list of `user`, `issue`, or `json` variable types.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<crate::models::CustomContextVariable>>,
}

impl JiraExpressionEvalContextBean {
    pub fn new() -> JiraExpressionEvalContextBean {
        JiraExpressionEvalContextBean {
            issue: None,
            issues: None,
            project: None,
            sprint: None,
            board: None,
            service_desk: None,
            customer_request: None,
            custom: None,
        }
    }
}



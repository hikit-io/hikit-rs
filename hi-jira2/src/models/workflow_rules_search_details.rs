/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowRulesSearchDetails : Details of workflow transition rules.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowRulesSearchDetails {
    /// The workflow ID.
    #[serde(rename = "workflowEntityId", skip_serializing_if = "Option::is_none")]
    pub workflow_entity_id: Option<String>,
    /// List of workflow rule IDs that do not belong to the workflow or can not be found.
    #[serde(rename = "invalidRules", skip_serializing_if = "Option::is_none")]
    pub invalid_rules: Option<Vec<String>>,
    /// List of valid workflow transition rules.
    #[serde(rename = "validRules", skip_serializing_if = "Option::is_none")]
    pub valid_rules: Option<Vec<crate::models::WorkflowTransitionRules>>,
}

impl WorkflowRulesSearchDetails {
    /// Details of workflow transition rules.
    pub fn new() -> WorkflowRulesSearchDetails {
        WorkflowRulesSearchDetails {
            workflow_entity_id: None,
            invalid_rules: None,
            valid_rules: None,
        }
    }
}


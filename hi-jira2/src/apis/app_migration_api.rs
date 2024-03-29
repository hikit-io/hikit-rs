/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`app_issue_field_value_update_resource_update_issue_fields_put`]
#[derive(Clone, Debug, Default)]
pub struct AppIssueFieldValueUpdateResourceUpdateIssueFieldsPutParams {
    /// The ID of the transfer.
    pub atlassian_transfer_id: String,
    /// The Atlassian account ID of the impersonated user. This user must be a member of the site admin group.
    pub atlassian_account_id: String,
    pub connect_custom_field_values: crate::models::ConnectCustomFieldValues
}

/// struct for passing parameters to the method [`migration_resource_update_entity_properties_value_put`]
#[derive(Clone, Debug, Default)]
pub struct MigrationResourceUpdateEntityPropertiesValuePutParams {
    /// The app migration transfer ID.
    pub atlassian_transfer_id: String,
    /// The Atlassian account ID of the impersonated user. This user must be a member of the site admin group.
    pub atlassian_account_id: String,
    /// The type indicating the object that contains the entity properties.
    pub entity_type: String,
    pub entity_property_details: Vec<crate::models::EntityPropertyDetails>
}

/// struct for passing parameters to the method [`migration_resource_workflow_rule_search_post`]
#[derive(Clone, Debug, Default)]
pub struct MigrationResourceWorkflowRuleSearchPostParams {
    /// The app migration transfer ID.
    pub atlassian_transfer_id: String,
    pub workflow_rules_search: crate::models::WorkflowRulesSearch
}


/// struct for typed errors of method [`app_issue_field_value_update_resource_update_issue_fields_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppIssueFieldValueUpdateResourceUpdateIssueFieldsPutError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`migration_resource_update_entity_properties_value_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MigrationResourceUpdateEntityPropertiesValuePutError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`migration_resource_workflow_rule_search_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MigrationResourceWorkflowRuleSearchPostError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}


/// Updates the value of a custom field added by Connect apps on one or more issues. The values of up to 200 custom fields can be updated.  **[Permissions](#permissions) required:** Only Connect apps can make this request.
pub async fn app_issue_field_value_update_resource_update_issue_fields_put(configuration: &configuration::Configuration, params: AppIssueFieldValueUpdateResourceUpdateIssueFieldsPutParams) -> Result<serde_json::Value, Error<AppIssueFieldValueUpdateResourceUpdateIssueFieldsPutError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let atlassian_transfer_id = params.atlassian_transfer_id;
    let atlassian_account_id = params.atlassian_account_id;
    let connect_custom_field_values = params.connect_custom_field_values;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/migration/field", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Atlassian-Transfer-Id", atlassian_transfer_id.to_string());
    local_var_req_builder = local_var_req_builder.header("Atlassian-Account-Id", atlassian_account_id.to_string());
    local_var_req_builder = local_var_req_builder.json(&connect_custom_field_values);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AppIssueFieldValueUpdateResourceUpdateIssueFieldsPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the values of multiple entity properties for an object, up to 50 updates per request. This operation is for use by Connect apps during app migration.
pub async fn migration_resource_update_entity_properties_value_put(configuration: &configuration::Configuration, params: MigrationResourceUpdateEntityPropertiesValuePutParams) -> Result<(), Error<MigrationResourceUpdateEntityPropertiesValuePutError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let atlassian_transfer_id = params.atlassian_transfer_id;
    let atlassian_account_id = params.atlassian_account_id;
    let entity_type = params.entity_type;
    let entity_property_details = params.entity_property_details;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/migration/properties/{entityType}", local_var_configuration.base_path, entityType=crate::apis::urlencode(entity_type));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Atlassian-Transfer-Id", atlassian_transfer_id.to_string());
    local_var_req_builder = local_var_req_builder.header("Atlassian-Account-Id", atlassian_account_id.to_string());
    local_var_req_builder = local_var_req_builder.json(&entity_property_details);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<MigrationResourceUpdateEntityPropertiesValuePutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns configurations for workflow transition rules migrated from server to cloud and owned by the calling Connect app.
pub async fn migration_resource_workflow_rule_search_post(configuration: &configuration::Configuration, params: MigrationResourceWorkflowRuleSearchPostParams) -> Result<crate::models::WorkflowRulesSearchDetails, Error<MigrationResourceWorkflowRuleSearchPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let atlassian_transfer_id = params.atlassian_transfer_id;
    let workflow_rules_search = params.workflow_rules_search;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/atlassian-connect/1/migration/workflow/rule/search", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Atlassian-Transfer-Id", atlassian_transfer_id.to_string());
    local_var_req_builder = local_var_req_builder.json(&workflow_rules_search);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MigrationResourceWorkflowRuleSearchPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


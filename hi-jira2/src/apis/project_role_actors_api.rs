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

/// struct for passing parameters to the method [`add_actor_users`]
#[derive(Clone, Debug, Default)]
pub struct AddActorUsersParams {
    /// The project ID or project key (case sensitive).
    pub project_id_or_key: String,
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-2-role-get) to get a list of project role IDs.
    pub id: i64,
    /// The groups or users to associate with the project role for this project. Provide the user account ID or group name.
    pub actors_map: crate::models::ActorsMap
}

/// struct for passing parameters to the method [`add_project_role_actors_to_role`]
#[derive(Clone, Debug, Default)]
pub struct AddProjectRoleActorsToRoleParams {
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-2-role-get) to get a list of project role IDs.
    pub id: i64,
    pub actor_input_bean: crate::models::ActorInputBean
}

/// struct for passing parameters to the method [`delete_actor`]
#[derive(Clone, Debug, Default)]
pub struct DeleteActorParams {
    /// The project ID or project key (case sensitive).
    pub project_id_or_key: String,
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-2-role-get) to get a list of project role IDs.
    pub id: i64,
    /// The user account ID of the user to remove from the project role.
    pub user: Option<String>,
    /// The name of the group to remove from the project role.
    pub group: Option<String>
}

/// struct for passing parameters to the method [`delete_project_role_actors_from_role`]
#[derive(Clone, Debug, Default)]
pub struct DeleteProjectRoleActorsFromRoleParams {
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-2-role-get) to get a list of project role IDs.
    pub id: i64,
    /// The user account ID of the user to remove as a default actor.
    pub user: Option<String>,
    /// The group name of the group to be removed as a default actor.
    pub group: Option<String>
}

/// struct for passing parameters to the method [`get_project_role_actors_for_role`]
#[derive(Clone, Debug, Default)]
pub struct GetProjectRoleActorsForRoleParams {
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-2-role-get) to get a list of project role IDs.
    pub id: i64
}

/// struct for passing parameters to the method [`set_actors`]
#[derive(Clone, Debug, Default)]
pub struct SetActorsParams {
    /// The project ID or project key (case sensitive).
    pub project_id_or_key: String,
    /// The ID of the project role. Use [Get all project roles](#api-rest-api-2-role-get) to get a list of project role IDs.
    pub id: i64,
    /// The groups or users to associate with the project role for this project. Provide the user account ID or group name.
    pub project_role_actors_update_bean: crate::models::ProjectRoleActorsUpdateBean
}


/// struct for typed errors of method [`add_actor_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddActorUsersError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_project_role_actors_to_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddProjectRoleActorsToRoleError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_actor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteActorError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_project_role_actors_from_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteProjectRoleActorsFromRoleError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_project_role_actors_for_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjectRoleActorsForRoleError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_actors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetActorsError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Adds actors to a project role for the project.  To replace all actors for the project, use [Set actors for project role](#api-rest-api-2-project-projectIdOrKey-role-id-put).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn add_actor_users(configuration: &configuration::Configuration, params: AddActorUsersParams) -> Result<crate::models::ProjectRole, Error<AddActorUsersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project_id_or_key = params.project_id_or_key;
    let id = params.id;
    let actors_map = params.actors_map;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/project/{projectIdOrKey}/role/{id}", local_var_configuration.base_path, projectIdOrKey=crate::apis::urlencode(project_id_or_key), id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&actors_map);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddActorUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Adds [default actors](#api-rest-api-2-resolution-get) to a role. You may add groups or users, but you cannot add groups and users in the same request.  Changing a project role's default actors does not affect project role members for projects already created.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn add_project_role_actors_to_role(configuration: &configuration::Configuration, params: AddProjectRoleActorsToRoleParams) -> Result<crate::models::ProjectRole, Error<AddProjectRoleActorsToRoleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let actor_input_bean = params.actor_input_bean;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/role/{id}/actors", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&actor_input_bean);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddProjectRoleActorsToRoleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes actors from a project role for the project.  To remove default actors from the project role, use [Delete default actors from project role](#api-rest-api-2-role-id-actors-delete).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_actor(configuration: &configuration::Configuration, params: DeleteActorParams) -> Result<(), Error<DeleteActorError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project_id_or_key = params.project_id_or_key;
    let id = params.id;
    let user = params.user;
    let group = params.group;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/project/{projectIdOrKey}/role/{id}", local_var_configuration.base_path, projectIdOrKey=crate::apis::urlencode(project_id_or_key), id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group {
        local_var_req_builder = local_var_req_builder.query(&[("group", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteActorError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the [default actors](#api-rest-api-2-resolution-get) from a project role. You may delete a group or user, but you cannot delete a group and a user in the same request.  Changing a project role's default actors does not affect project role members for projects already created.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_project_role_actors_from_role(configuration: &configuration::Configuration, params: DeleteProjectRoleActorsFromRoleParams) -> Result<crate::models::ProjectRole, Error<DeleteProjectRoleActorsFromRoleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let user = params.user;
    let group = params.group;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/role/{id}/actors", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group {
        local_var_req_builder = local_var_req_builder.query(&[("group", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteProjectRoleActorsFromRoleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the [default actors](#api-rest-api-2-resolution-get) for the project role.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn get_project_role_actors_for_role(configuration: &configuration::Configuration, params: GetProjectRoleActorsForRoleParams) -> Result<crate::models::ProjectRole, Error<GetProjectRoleActorsForRoleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/role/{id}/actors", local_var_configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetProjectRoleActorsForRoleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the actors for a project role for a project, replacing all existing actors.  To add actors to the project without overwriting the existing list, use [Add actors to project role](#api-rest-api-2-project-projectIdOrKey-role-id-post).  **[Permissions](#permissions) required:** *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn set_actors(configuration: &configuration::Configuration, params: SetActorsParams) -> Result<crate::models::ProjectRole, Error<SetActorsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project_id_or_key = params.project_id_or_key;
    let id = params.id;
    let project_role_actors_update_bean = params.project_role_actors_update_bean;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/rest/api/2/project/{projectIdOrKey}/role/{id}", local_var_configuration.base_path, projectIdOrKey=crate::apis::urlencode(project_id_or_key), id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&project_role_actors_update_bean);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetActorsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


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
pub struct ProjectLandingPageInfo {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "projectKey", skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "projectType", skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    #[serde(rename = "boardId", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i64>,
    #[serde(rename = "simplified", skip_serializing_if = "Option::is_none")]
    pub simplified: Option<bool>,
}

impl ProjectLandingPageInfo {
    pub fn new() -> ProjectLandingPageInfo {
        ProjectLandingPageInfo {
            url: None,
            project_key: None,
            project_type: None,
            board_id: None,
            simplified: None,
        }
    }
}



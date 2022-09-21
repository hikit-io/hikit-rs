/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardDetails : Details of a dashboard.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardDetails {
    /// The name of the dashboard.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the dashboard.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The share permissions for the dashboard.
    #[serde(rename = "sharePermissions")]
    pub share_permissions: Vec<crate::models::SharePermission>,
    /// The edit permissions for the dashboard.
    #[serde(rename = "editPermissions")]
    pub edit_permissions: Vec<crate::models::SharePermission>,
}

impl DashboardDetails {
    /// Details of a dashboard.
    pub fn new(name: String, share_permissions: Vec<crate::models::SharePermission>, edit_permissions: Vec<crate::models::SharePermission>) -> DashboardDetails {
        DashboardDetails {
            name,
            description: None,
            share_permissions,
            edit_permissions,
        }
    }
}


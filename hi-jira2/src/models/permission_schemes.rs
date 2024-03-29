/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PermissionSchemes : List of all permission schemes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionSchemes {
    /// Permission schemes list.
    #[serde(rename = "permissionSchemes", skip_serializing_if = "Option::is_none")]
    pub permission_schemes: Option<Vec<crate::models::PermissionScheme>>,
}

impl PermissionSchemes {
    /// List of all permission schemes.
    pub fn new() -> PermissionSchemes {
        PermissionSchemes {
            permission_schemes: None,
        }
    }
}



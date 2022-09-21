/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SimpleLink : Details about the operations available in this version.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimpleLink {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "styleClass", skip_serializing_if = "Option::is_none")]
    pub style_class: Option<String>,
    #[serde(rename = "iconClass", skip_serializing_if = "Option::is_none")]
    pub icon_class: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl SimpleLink {
    /// Details about the operations available in this version.
    pub fn new() -> SimpleLink {
        SimpleLink {
            id: None,
            style_class: None,
            icon_class: None,
            label: None,
            title: None,
            href: None,
            weight: None,
        }
    }
}


/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardGadget : Details of a gadget.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardGadget {
    /// The ID of the gadget instance.
    #[serde(rename = "id")]
    pub id: i64,
    /// The module key of the gadget type.
    #[serde(rename = "moduleKey", skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    /// The URI of the gadget type.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    #[serde(rename = "color")]
    pub color: Color,
    /// The position of the gadget.
    #[serde(rename = "position")]
    pub position: Option<crate::models::DashboardGadgetPosition>,
    /// The title of the gadget.
    #[serde(rename = "title")]
    pub title: String,
}

impl DashboardGadget {
    /// Details of a gadget.
    pub fn new(id: i64, color: Color, position: Option<crate::models::DashboardGadgetPosition>, title: String) -> DashboardGadget {
        DashboardGadget {
            id,
            module_key: None,
            uri: None,
            color,
            position: position,
            title,
        }
    }
}

/// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "white")]
    White,
}

impl Default for Color {
    fn default() -> Color {
        Self::Blue
    }
}

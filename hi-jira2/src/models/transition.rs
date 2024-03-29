/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Transition : Details of a workflow transition.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transition {
    /// The ID of the transition.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the transition.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the transition.
    #[serde(rename = "description")]
    pub description: String,
    /// The statuses the transition can start from.
    #[serde(rename = "from")]
    pub from: Vec<String>,
    /// The status the transition goes to.
    #[serde(rename = "to")]
    pub to: String,
    /// The type of the transition.
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "screen", skip_serializing_if = "Option::is_none")]
    pub screen: Option<crate::models::TransitionScreenDetails>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<crate::models::WorkflowRules>,
    /// The properties of the transition.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl Transition {
    /// Details of a workflow transition.
    pub fn new(id: String, name: String, description: String, from: Vec<String>, to: String, _type: Type) -> Transition {
        Transition {
            id,
            name,
            description,
            from,
            to,
            _type,
            screen: None,
            rules: None,
            properties: None,
        }
    }
}

/// The type of the transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "initial")]
    Initial,
    #[serde(rename = "directed")]
    Directed,
}

impl Default for Type {
    fn default() -> Type {
        Self::Global
    }
}


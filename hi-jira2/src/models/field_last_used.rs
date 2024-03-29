/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldLastUsed : Information about the most recent use of a field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldLastUsed {
    /// Last used value type:   *  *TRACKED*: field is tracked and a last used date is available.  *  *NOT\\_TRACKED*: field is not tracked, last used date is not available.  *  *NO\\_INFORMATION*: field is tracked, but no last used date is available.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The date when the value of the field last changed.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl FieldLastUsed {
    /// Information about the most recent use of a field.
    pub fn new() -> FieldLastUsed {
        FieldLastUsed {
            _type: None,
            value: None,
        }
    }
}

/// Last used value type:   *  *TRACKED*: field is tracked and a last used date is available.  *  *NOT\\_TRACKED*: field is not tracked, last used date is not available.  *  *NO\\_INFORMATION*: field is tracked, but no last used date is available.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TRACKED")]
    TRACKED,
    #[serde(rename = "NOT_TRACKED")]
    NOTTRACKED,
    #[serde(rename = "NO_INFORMATION")]
    NOINFORMATION,
}

impl Default for Type {
    fn default() -> Type {
        Self::TRACKED
    }
}


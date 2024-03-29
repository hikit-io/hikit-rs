/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldValueUpdate : A list of issue IDs and the value to update a custom field to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldValueUpdate {
    /// The list of issue IDs.
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<i64>,
    /// The value for the custom field. The value must be compatible with the [custom field type](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/#data-types) as follows:   *  `string` the value must be a string.  *  `number` the value must be a number.  *  `datetime` the value must be a string that represents a date in the ISO format or the simplified extended ISO format. For example, `\"2023-01-18T12:00:00-03:00\"` or `\"2023-01-18T12:00:00.000Z\"`. However, the milliseconds part is ignored.  *  `user` the value must be an object that contains the `accountId` field.  *  `group` the value must be an object that contains the group `name` field.  A list of appropriate values must be provided if the field is of the `list` [collection type](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/#collection-types).
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
}

impl CustomFieldValueUpdate {
    /// A list of issue IDs and the value to update a custom field to.
    pub fn new(issue_ids: Vec<i64>, value: Option<serde_json::Value>) -> CustomFieldValueUpdate {
        CustomFieldValueUpdate {
            issue_ids,
            value,
        }
    }
}



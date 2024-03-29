/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CustomFieldContextDefaultValue {
    #[serde(rename="datepicker")]
    CustomFieldContextDefaultValueDate {
        /// The default date in ISO format. Ignored if `useCurrent` is true.
        #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
        date: Option<String>,
        /// Whether to use the current date.
        #[serde(rename = "useCurrent", skip_serializing_if = "Option::is_none")]
        use_current: Option<bool>,
    },
    #[serde(rename="datetimepicker")]
    CustomFieldContextDefaultValueDateTime {
        /// The default date-time in ISO format. Ignored if `useCurrent` is true.
        #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
        date_time: Option<String>,
        /// Whether to use the current date.
        #[serde(rename = "useCurrent", skip_serializing_if = "Option::is_none")]
        use_current: Option<bool>,
    },
    #[serde(rename="float")]
    CustomFieldContextDefaultValueFloat {
        /// The default floating-point number.
        #[serde(rename = "number")]
        number: f64,
    },
    #[serde(rename="forge.datetime")]
    CustomFieldContextDefaultValueForgeDateTimeField {
        /// The default date-time in ISO format. Ignored if `useCurrent` is true.
        #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
        date_time: Option<String>,
        /// Whether to use the current date.
        #[serde(rename = "useCurrent", skip_serializing_if = "Option::is_none")]
        use_current: Option<bool>,
    },
    #[serde(rename="forge.group")]
    CustomFieldContextDefaultValueForgeGroupField {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The ID of the the default group.
        #[serde(rename = "groupId")]
        group_id: String,
    },
    #[serde(rename="forge.group.list")]
    CustomFieldContextDefaultValueForgeMultiGroupField {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The IDs of the default groups.
        #[serde(rename = "groupIds")]
        group_ids: Vec<String>,
    },
    #[serde(rename="forge.number")]
    CustomFieldContextDefaultValueForgeNumberField {
        /// The default floating-point number.
        #[serde(rename = "number")]
        number: f64,
    },
    #[serde(rename="forge.object")]
    CustomFieldContextDefaultValueForgeObjectField {
        /// The default JSON object.
        #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
        object: Option<serde_json::Value>,
    },
    #[serde(rename="forge.string")]
    CustomFieldContextDefaultValueForgeStringField {
        /// The default text. The maximum length is 254 characters.
        #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    #[serde(rename="forge.string.list")]
    CustomFieldContextDefaultValueForgeMultiStringField {
        /// List of string values. The maximum length for a value is 254 characters.
        #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
        values: Option<Vec<String>>,
    },
    #[serde(rename="forge.user")]
    CustomFieldContextDefaultValueForgeUserField {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The ID of the default user.
        #[serde(rename = "accountId")]
        account_id: String,
        #[serde(rename = "userFilter")]
        user_filter: crate::models::UserFilter,
    },
    #[serde(rename="forge.user.list")]
    CustomFieldContextDefaultValueForgeMultiUserField {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The IDs of the default users.
        #[serde(rename = "accountIds")]
        account_ids: Vec<String>,
    },
    #[serde(rename="grouppicker.multiple")]
    CustomFieldContextDefaultValueMultipleGroupPicker {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The IDs of the default groups.
        #[serde(rename = "groupIds")]
        group_ids: Vec<String>,
    },
    #[serde(rename="grouppicker.single")]
    CustomFieldContextDefaultValueSingleGroupPicker {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The ID of the the default group.
        #[serde(rename = "groupId")]
        group_id: String,
    },
    #[serde(rename="labels")]
    CustomFieldContextDefaultValueLabels {
        /// The default labels value.
        #[serde(rename = "labels")]
        labels: Vec<String>,
    },
    #[serde(rename="multi.user.select")]
    CustomFieldContextDefaultValueMultiUserPicker {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The IDs of the default users.
        #[serde(rename = "accountIds")]
        account_ids: Vec<String>,
    },
    #[serde(rename="option.cascading")]
    CustomFieldContextDefaultValueCascadingOption {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The ID of the default option.
        #[serde(rename = "optionId")]
        option_id: String,
        /// The ID of the default cascading option.
        #[serde(rename = "cascadingOptionId", skip_serializing_if = "Option::is_none")]
        cascading_option_id: Option<String>,
    },
    #[serde(rename="option.multiple")]
    CustomFieldContextDefaultValueMultipleOption {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The list of IDs of the default options.
        #[serde(rename = "optionIds")]
        option_ids: Vec<String>,
    },
    #[serde(rename="option.single")]
    CustomFieldContextDefaultValueSingleOption {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The ID of the default option.
        #[serde(rename = "optionId")]
        option_id: String,
    },
    #[serde(rename="project")]
    CustomFieldContextDefaultValueProject {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The ID of the default project.
        #[serde(rename = "projectId")]
        project_id: String,
    },
    #[serde(rename="readonly")]
    CustomFieldContextDefaultValueReadOnly {
        /// The default text. The maximum length is 255 characters.
        #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    #[serde(rename="single.user.select")]
    CustomFieldContextSingleUserPickerDefaults {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The ID of the default user.
        #[serde(rename = "accountId")]
        account_id: String,
        #[serde(rename = "userFilter")]
        user_filter: crate::models::UserFilter,
    },
    #[serde(rename="textarea")]
    CustomFieldContextDefaultValueTextArea {
        /// The default text. The maximum length is 32767 characters.
        #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    #[serde(rename="textfield")]
    CustomFieldContextDefaultValueTextField {
        /// The default text. The maximum length is 254 characters.
        #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    #[serde(rename="url")]
    CustomFieldContextDefaultValueUrl {
        /// The ID of the context.
        #[serde(rename = "contextId")]
        context_id: String,
        /// The default URL.
        #[serde(rename = "url")]
        url: String,
    },
    #[serde(rename="version.multiple")]
    CustomFieldContextDefaultValueMultipleVersionPicker {
        /// The IDs of the default versions.
        #[serde(rename = "versionIds")]
        version_ids: Vec<String>,
        /// The order the pickable versions are displayed in. If not provided, the released-first order is used. Available version orders are `\"releasedFirst\"` and `\"unreleasedFirst\"`.
        #[serde(rename = "versionOrder", skip_serializing_if = "Option::is_none")]
        version_order: Option<String>,
    },
    #[serde(rename="version.single")]
    CustomFieldContextDefaultValueSingleVersionPicker {
        /// The ID of the default version.
        #[serde(rename = "versionId")]
        version_id: String,
        /// The order the pickable versions are displayed in. If not provided, the released-first order is used. Available version orders are `\"releasedFirst\"` and `\"unreleasedFirst\"`.
        #[serde(rename = "versionOrder", skip_serializing_if = "Option::is_none")]
        version_order: Option<String>,
    },
}





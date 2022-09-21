# CustomFieldContextDefaultValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_id** | **String** | The ID of the context. | 
**option_id** | **String** | The ID of the default option. | 
**cascading_option_id** | Option<**String**> | The ID of the default cascading option. | [optional]
**_type** | **String** |  | 
**option_ids** | **Vec<String>** | The list of IDs of the default options. | 
**account_id** | **String** | The ID of the default user. | 
**user_filter** | [**crate::models::UserFilter**](UserFilter.md) |  | 
**account_ids** | **Vec<String>** | The IDs of the default users. | 
**group_id** | **String** | The ID of the the default group. | 
**group_ids** | **Vec<String>** | The IDs of the default groups. | 
**date** | Option<**String**> | The default date in ISO format. Ignored if `useCurrent` is true. | [optional]
**use_current** | Option<**bool**> | Whether to use the current date. | [optional][default to false]
**date_time** | Option<**String**> | The default date-time in ISO format. Ignored if `useCurrent` is true. | [optional]
**url** | **String** | The default URL. | 
**project_id** | **String** | The ID of the default project. | 
**number** | **f64** | The default floating-point number. | 
**labels** | **Vec<String>** | The default labels value. | 
**text** | Option<**String**> | The default text. The maximum length is 254 characters. | [optional]
**version_id** | **String** | The ID of the default version. | 
**version_order** | Option<**String**> | The order the pickable versions are displayed in. If not provided, the released-first order is used. Available version orders are `\"releasedFirst\"` and `\"unreleasedFirst\"`. | [optional]
**version_ids** | **Vec<String>** | The IDs of the default versions. | 
**values** | Option<**Vec<String>**> | List of string values. The maximum length for a value is 254 characters. | [optional]
**object** | Option<[**serde_json::Value**](.md)> | The default JSON object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



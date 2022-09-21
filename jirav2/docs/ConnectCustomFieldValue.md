# ConnectCustomFieldValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The type of custom field. | 
**issue_id** | **i32** | The issue ID. | 
**field_id** | **i32** | The custom field ID. | 
**string** | Option<**String**> | The value of string type custom field when `_type` is `StringIssueField`. | [optional]
**number** | Option<**f32**> | The value of number type custom field when `_type` is `NumberIssueField`. | [optional]
**rich_text** | Option<**String**> | The value of richText type custom field when `_type` is `RichTextIssueField`. | [optional]
**option_id** | Option<**String**> | The value of single select and multiselect custom field type when `_type` is `SingleSelectIssueField` or `MultiSelectIssueField`. | [optional]
**text** | Option<**String**> | The value of of text custom field type when `_type` is `TextIssueField`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



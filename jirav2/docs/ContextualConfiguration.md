# ContextualConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the configuration. | 
**context_id** | Option<**i64**> | Deprecated, do not use. | [optional]
**field_context_id** | **String** | The ID of the field context the configuration is associated with. | [readonly]
**configuration** | Option<[**serde_json::Value**](.md)> | The field configuration. | [optional]
**schema** | Option<[**serde_json::Value**](.md)> | The field value schema. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# \AppMigrationApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_issue_field_value_update_resource_update_issue_fields_put**](AppMigrationApi.md#app_issue_field_value_update_resource_update_issue_fields_put) | **PUT** /rest/atlassian-connect/1/migration/field | Bulk update custom field value
[**migration_resource_update_entity_properties_value_put**](AppMigrationApi.md#migration_resource_update_entity_properties_value_put) | **PUT** /rest/atlassian-connect/1/migration/properties/{entityType} | Bulk update entity properties
[**migration_resource_workflow_rule_search_post**](AppMigrationApi.md#migration_resource_workflow_rule_search_post) | **POST** /rest/atlassian-connect/1/migration/workflow/rule/search | Get workflow transition rule configurations



## app_issue_field_value_update_resource_update_issue_fields_put

> serde_json::Value app_issue_field_value_update_resource_update_issue_fields_put(atlassian_transfer_id, atlassian_account_id, connect_custom_field_values)
Bulk update custom field value

Updates the value of a custom field added by Connect apps on one or more issues. The values of up to 200 custom fields can be updated.  **[Permissions](#permissions) required:** Only Connect apps can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**atlassian_transfer_id** | **String** | The ID of the transfer. | [required] |
**atlassian_account_id** | **String** | The Atlassian account ID of the impersonated user. This user must be a member of the site admin group. | [required] |
**connect_custom_field_values** | [**ConnectCustomFieldValues**](ConnectCustomFieldValues.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_resource_update_entity_properties_value_put

> migration_resource_update_entity_properties_value_put(atlassian_transfer_id, atlassian_account_id, entity_type, entity_property_details)
Bulk update entity properties

Updates the values of multiple entity properties for an object, up to 50 updates per request. This operation is for use by Connect apps during app migration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**atlassian_transfer_id** | **String** | The app migration transfer ID. | [required] |
**atlassian_account_id** | **String** | The Atlassian account ID of the impersonated user. This user must be a member of the site admin group. | [required] |
**entity_type** | **String** | The type indicating the object that contains the entity properties. | [required] |
**entity_property_details** | [**Vec<crate::models::EntityPropertyDetails>**](EntityPropertyDetails.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_resource_workflow_rule_search_post

> crate::models::WorkflowRulesSearchDetails migration_resource_workflow_rule_search_post(atlassian_transfer_id, workflow_rules_search)
Get workflow transition rule configurations

Returns configurations for workflow transition rules migrated from server to cloud and owned by the calling Connect app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**atlassian_transfer_id** | **String** | The app migration transfer ID. | [required] |
**workflow_rules_search** | [**WorkflowRulesSearch**](WorkflowRulesSearch.md) |  | [required] |

### Return type

[**crate::models::WorkflowRulesSearchDetails**](WorkflowRulesSearchDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


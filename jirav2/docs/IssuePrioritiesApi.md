# \IssuePrioritiesApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_priorities**](IssuePrioritiesApi.md#get_priorities) | **GET** /rest/api/2/priority | Get priorities
[**get_priority**](IssuePrioritiesApi.md#get_priority) | **GET** /rest/api/2/priority/{id} | Get priority



## get_priorities

> Vec<crate::models::Priority> get_priorities()
Get priorities

Returns the list of all issue priorities.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Priority>**](Priority.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_priority

> crate::models::Priority get_priority(id)
Get priority

Returns an issue priority.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue priority. | [required] |

### Return type

[**crate::models::Priority**](Priority.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


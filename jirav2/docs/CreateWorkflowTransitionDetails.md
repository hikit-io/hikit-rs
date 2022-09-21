# CreateWorkflowTransitionDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the transition. The maximum length is 60 characters. | 
**description** | Option<**String**> | The description of the transition. The maximum length is 1000 characters. | [optional]
**from** | Option<**Vec<String>**> | The statuses the transition can start from. | [optional]
**to** | **String** | The status the transition goes to. | 
**_type** | **String** | The type of the transition. | 
**rules** | Option<[**crate::models::CreateWorkflowTransitionRulesDetails**](CreateWorkflowTransitionRulesDetails.md)> | The rules of the transition. | [optional]
**screen** | Option<[**crate::models::CreateWorkflowTransitionScreenDetails**](CreateWorkflowTransitionScreenDetails.md)> | The screen of the transition. | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | The properties of the transition. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



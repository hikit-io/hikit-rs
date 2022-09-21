# WorkflowRulesSearchDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_entity_id** | Option<**String**> | The workflow ID. | [optional]
**invalid_rules** | Option<**Vec<String>**> | List of workflow rule IDs that do not belong to the workflow or can not be found. | [optional]
**valid_rules** | Option<[**Vec<crate::models::WorkflowTransitionRules>**](WorkflowTransitionRules.md)> | List of valid workflow transition rules. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



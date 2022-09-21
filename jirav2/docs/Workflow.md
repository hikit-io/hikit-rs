# Workflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**crate::models::PublishedWorkflowId**](PublishedWorkflowId.md) |  | 
**description** | **String** | The description of the workflow. | 
**transitions** | Option<[**Vec<crate::models::Transition>**](Transition.md)> | The transitions of the workflow. | [optional]
**statuses** | Option<[**Vec<crate::models::WorkflowStatus>**](WorkflowStatus.md)> | The statuses of the workflow. | [optional]
**is_default** | Option<**bool**> | Whether this is the default workflow. | [optional]
**schemes** | Option<[**Vec<crate::models::WorkflowSchemeIdName>**](WorkflowSchemeIdName.md)> | The workflow schemes the workflow is assigned to. | [optional]
**projects** | Option<[**Vec<crate::models::ProjectDetails>**](ProjectDetails.md)> | The projects the workflow is assigned to, through workflow schemes. | [optional]
**has_draft_workflow** | Option<**bool**> | Whether the workflow has a draft version. | [optional]
**operations** | Option<[**crate::models::WorkflowOperations**](WorkflowOperations.md)> |  | [optional]
**created** | Option<**String**> | The creation date of the workflow. | [optional]
**updated** | Option<**String**> | The last edited date of the workflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



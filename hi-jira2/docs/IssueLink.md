# IssueLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the issue link. | [optional][readonly]
**_self** | Option<**String**> | The URL of the issue link. | [optional][readonly]
**_type** | Option<[**crate::models::IssueLinkType**](IssueLinkType.md)> | The type of link between the issues. | 
**inward_issue** | Option<[**crate::models::LinkedIssue**](LinkedIssue.md)> | Provides details about the linked issue. If presenting this link in a user interface, use the `inward` field of the issue link type to label the link. | 
**outward_issue** | Option<[**crate::models::LinkedIssue**](LinkedIssue.md)> | Provides details about the linked issue. If presenting this link in a user interface, use the `outward` field of the issue link type to label the link. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



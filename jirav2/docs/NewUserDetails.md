# NewUserDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_self** | Option<**String**> | The URL of the user. | [optional][readonly]
**key** | Option<**String**> | This property is no longer available. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional]
**name** | Option<**String**> | This property is no longer available. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. | [optional]
**password** | Option<**String**> | This property is no longer available. If the user has an Atlassian account, their password is not changed. If the user does not have an Atlassian account, they are sent an email asking them set up an account. | [optional]
**email_address** | **String** | The email address for the user. | 
**display_name** | **String** | A suggested display name for the user. If the user has an Atlassian account, their display name is not changed. If the user does not have an Atlassian account, this display name is used as a suggestion for creating an account. The user is sent an email asking them to set their display name and privacy preferences. | 
**application_keys** | Option<**Vec<String>**> | Deprecated, do not use. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



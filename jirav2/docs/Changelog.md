# Changelog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the changelog. | [optional][readonly]
**author** | Option<[**crate::models::UserDetails**](UserDetails.md)> | The user who made the change. | [optional][readonly]
**created** | Option<**String**> | The date on which the change took place. | [optional][readonly]
**items** | Option<[**Vec<crate::models::ChangeDetails>**](ChangeDetails.md)> | The list of items changed. | [optional][readonly]
**history_metadata** | Option<[**crate::models::HistoryMetadata**](HistoryMetadata.md)> | The history metadata associated with the changed. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



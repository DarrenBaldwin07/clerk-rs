# CreateOrganizationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the new organization | 
**created_by** | Option<**String**> | The ID of the User who will become the administrator for the new organization | [optional]
**private_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the organization, accessible only from the Backend API | [optional]
**public_metadata** | Option<[**serde_json::Value**](.md)> | Metadata saved on the organization, read-only from the Frontend API and fully accessible (read/write) from the Backend API | [optional]
**slug** | Option<**String**> | A slug for the new organization. Can contain only lowercase alphanumeric characters and the dash \"-\". Must be unique for the instance. | [optional]
**max_allowed_memberships** | Option<**i32**> | The maximum number of memberships allowed for this organization | [optional]
**created-at** | Option<**String**> | A custom date/time denoting _when_ the organization was created, specified in RFC3339 format (e.g. `2012-10-20T07:15:20.902Z`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



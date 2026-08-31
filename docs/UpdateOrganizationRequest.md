# UpdateOrganizationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The new name of the organization. May not contain URLs or HTML. Max length: 256 | [optional]
**slug** | Option<**String**> | The new slug of the organization, which needs to be unique in the instance | [optional]
**max_allowed_memberships** | Option<**u32**> | The maximum number of memberships allowed for this organization | [optional]
**admin_delete_enabled** | Option<**bool**> | If true, an admin can delete this organization with the Frontend API. | [optional]
**created_at** | Option<**String**> | A custom date/time denoting _when_ the organization was created, specified in RFC3339 format (e.g. `2012-10-20T07:15:20.902Z`). | [optional]
**role_set_key** | Option<**String**> | The key of the [role set](https://clerk.com/docs/guides/organizations/control-access/role-sets) to assign to this organization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



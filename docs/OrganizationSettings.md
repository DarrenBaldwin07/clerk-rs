# OrganizationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: organization_settings) |
**enabled** | **bool** |  |
**max_allowed_memberships** | **i32** |  |
**max_allowed_roles** | **i32** |  |
**max_role_sets_allowed** | Option<**i32**> |  | [optional]
**max_allowed_domains** | **i32** |  |
**max_allowed_permissions** | Option<**i32**> | max_allowed_permissions is now a no-op, as permissions are now unlimited | [optional]
**creator_role** | **String** | The role key that a user will be assigned after creating an organization. |
**admin_delete_enabled** | **bool** | The default for whether an admin can delete an organization with the Frontend API. |
**domains_enabled** | **bool** |  |
**slug_disabled** | Option<**bool**> |  | [optional]
**domains_enrollment_modes** | **Vec<DomainsEnrollmentModes>** |  (enum: manual_invitation, automatic_invitation, automatic_suggestion) |
**domains_default_role** | **String** | The role key that it will be used in order to create an organization invitation or suggestion. |
**initial_role_set_key** | Option<**String**> | The role set key that it will be used to create new organizations. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



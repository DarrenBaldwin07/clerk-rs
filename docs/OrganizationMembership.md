# OrganizationMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  |
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: organization_membership) |
**role** | **String** |  |
**role_name** | Option<**String**> |  | [optional]
**permissions** | **Vec<String>** |  |
**public_metadata** | **std::collections::HashMap<String, serde_json::Value>** | Metadata saved on the organization membership, accessible from both Frontend and Backend APIs |
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Metadata saved on the organization membership, accessible only from the Backend API | [optional]
**organization** | [**models::Organization**](Organization.md) |  |
**public_user_data** | Option<[**models::OrganizationMembershipPublicUserData**](OrganizationMembershipPublicUserData.md)> |  | [optional]
**created_at** | **i64** | Unix timestamp of creation. |
**updated_at** | **i64** | Unix timestamp of last update. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



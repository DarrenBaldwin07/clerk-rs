# OrganizationInvitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: organization_invitation) |
**id** | **String** |  |
**email_address** | **String** |  |
**role** | **String** |  |
**role_name** | **String** |  |
**organization_id** | Option<**String**> |  | [optional]
**inviter_id** | Option<**String**> |  |
**public_inviter_data** | Option<[**models::OrganizationInvitationPublicUserData**](OrganizationInvitationPublicUserData.md)> |  |
**status** | Option<**String**> |  | [optional]
**public_metadata** | **std::collections::HashMap<String, serde_json::Value>** |  |
**private_metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**url** | Option<**String**> |  |
**expires_at** | Option<**i64**> | Unix timestamp of expiration. |
**created_at** | **i64** | Unix timestamp of creation. |
**updated_at** | **i64** | Unix timestamp of last update. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



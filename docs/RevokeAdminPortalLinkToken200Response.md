# RevokeAdminPortalLinkToken200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** |  (enum: admin_portal_link_token) |
**id** | **String** |  |
**admin_portal_link_id** | **String** | The origin-level admin_portal_link this token was issued for. |
**instance_id** | **String** |  |
**organization_id** | Option<**String**> | Optional. Present only when the link is scoped to an existing org. |
**it_contact_id** | Option<**String**> | Optional opaque reference to the IT contact associated with this link. |
**scopes** | Option<**Vec<String>**> | Caller-provided scopes for this token. |
**revoked** | **bool** |  |
**revocation_reason** | Option<**String**> |  |
**expired** | **bool** |  |
**expiration** | Option<**f64**> | The timestamp for when the token will expire, in milliseconds. |
**created_at** | **f64** | The timestamp for when the token was created, in milliseconds. |
**updated_at** | **f64** | The timestamp for when the token was last updated, in milliseconds. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



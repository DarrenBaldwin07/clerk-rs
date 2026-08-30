# EmailAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: email_address) |
**email_address** | **String** |  |
**reserved** | **bool** |  |
**verification** | Option<[**models::EmailAddressVerification**](EmailAddressVerification.md)> |  |
**linked_to** | [**Vec<models::IdentificationLink>**](IdentificationLink.md) |  |
**matches_sso_connection** | Option<**bool**> | Indicates whether this email address domain matches an active enterprise connection.  | [optional]
**created_at** | **i64** | Unix timestamp of creation  |
**updated_at** | **i64** | Unix timestamp of creation  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



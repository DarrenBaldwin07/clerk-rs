# PhoneNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**object** | **Object** | String representing the object's type. Objects of the same type share the same value.  (enum: phone_number) |
**phone_number** | **String** |  |
**reserved_for_second_factor** | Option<**bool**> |  | [optional]
**default_second_factor** | Option<**bool**> |  | [optional]
**reserved** | **bool** |  |
**verification** | Option<[**models::PhoneNumberVerification**](PhoneNumberVerification.md)> |  |
**linked_to** | [**Vec<models::IdentificationLink>**](IdentificationLink.md) |  |
**backup_codes** | Option<**Vec<String>**> |  | [optional]
**created_at** | **i64** | Unix timestamp of creation  |
**updated_at** | **i64** | Unix timestamp of creation  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



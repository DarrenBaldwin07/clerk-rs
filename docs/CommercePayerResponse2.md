# CommercePayerResponse2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_payer) |
**id** | **String** | Unique identifier for the payer. |
**instance_id** | **String** | Unique identifier for the Clerk instance. |
**user_id** | Option<**String**> | User ID for user-type payers. | [optional]
**first_name** | **String** | First name of the payer. |
**last_name** | **String** | Last name of the payer. |
**email** | **String** | Email address of the payer. |
**organization_id** | Option<**String**> | Organization ID for org-type payers. | [optional]
**organization_name** | Option<**String**> | Organization name for org-type payers. | [optional]
**image_url** | **String** | URL of the payer's image/avatar. |
**credits_balance** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  | [optional]
**created_at** | **i64** | Unix timestamp (in milliseconds) when the payer was created. |
**updated_at** | **i64** | Unix timestamp (in milliseconds) when the payer was last updated. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



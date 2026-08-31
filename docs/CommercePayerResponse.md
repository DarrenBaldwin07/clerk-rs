# CommercePayerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_payer) |
**id** | **String** | Unique identifier for the payer. |
**instance_id** | **String** | Unique identifier for the Clerk instance. |
**user_id** | Option<**String**> | User ID for user-type payers. | [optional]
**first_name** | Option<**String**> | First name of the payer. | [optional]
**last_name** | Option<**String**> | Last name of the payer. | [optional]
**email** | Option<**String**> | Email address of the payer. | [optional]
**organization_id** | Option<**String**> | Organization ID for org-type payers. | [optional]
**organization_name** | Option<**String**> | Organization name for org-type payers. | [optional]
**image_url** | Option<**String**> | URL of the payer's image/avatar. | [optional]
**credits_balance** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  | [optional]
**created_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the payer was created. | [optional]
**updated_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the payer was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



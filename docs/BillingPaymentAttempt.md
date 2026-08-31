# BillingPaymentAttempt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_payment) |
**id** | **String** | Unique identifier for the payment attempt. |
**payment_id** | **String** | Unique identifier for the associated payment. |
**instance_id** | **String** | The ID of the instance this payment attempt belongs to. |
**charge_type** | **String** | Type of charge for this payment attempt. |
**payee_id** | **String** | Unique identifier for the payee. |
**payee** | **serde_json::Value** | The payee associated with this payment attempt. |
**payer_id** | **String** | Unique identifier for the payer. |
**payer** | [**models::CommercePayerResponse**](CommercePayerResponse.md) |  |
**subscription_item_id** | Option<**String**> | Unique identifier for the associated subscription item. | [optional]
**subscription_item** | Option<**serde_json::Value**> | The subscription item associated with this payment attempt. | [optional]
**amount** | [**models::CommerceMoneyResponse**](CommerceMoneyResponse.md) |  |
**totals** | Option<[**models::CommerceTotalsResponse2**](CommerceTotalsResponse2.md)> | Totals breakdown for this payment attempt. | [optional]
**payment_method_id** | **String** | Unique identifier for the payment method. |
**payment_method** | [**models::CommercePaymentMethodResponse**](CommercePaymentMethodResponse.md) |  |
**statement_id** | **String** | Unique identifier for the associated statement. |
**gateway_external_id** | Option<**String**> | External identifier from the payment gateway. |
**gateway_external_url** | Option<**String**> | External URL from the payment gateway. |
**status** | **Status** | The current status of the payment attempt. (enum: pending, paid, failed) |
**paid_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the payment was completed. |
**failed_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the payment failed to be processed. |
**created_at** | **i64** | Unix timestamp (in milliseconds) when the payment attempt was created. |
**updated_at** | **i64** | Unix timestamp (in milliseconds) when the payment attempt was last updated. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



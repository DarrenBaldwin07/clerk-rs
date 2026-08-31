# CommercePaymentMethodResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_payment_method) |
**id** | **String** | Unique identifier for the payment method. |
**payer_id** | **String** | Unique identifier for the payer. |
**payment_type** | **PaymentType** | The payment method type. (enum: card, link, payer-credits) |
**is_default** | Option<**bool**> | Whether this is the default payment method for the payer. | [optional]
**gateway** | **String** | The payment gateway. |
**gateway_external_id** | **String** | External ID in the payment gateway. |
**gateway_external_account_id** | Option<**String**> | External account ID in the payment gateway. |
**last4** | Option<**String**> | Last 4 digits of the card (for card payment methods). |
**status** | **Status** | Status of the payment method. (enum: active, disconnected) |
**wallet_type** | Option<**String**> | Type of wallet (if applicable). | [optional]
**card_type** | Option<**String**> | Type of card (if applicable). |
**expiry_year** | Option<**i32**> | Card expiration year (for card payment methods). | [optional]
**expiry_month** | Option<**i32**> | Card expiration month (for card payment methods). | [optional]
**created_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the payment method was created. | [optional]
**updated_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the payment method was last updated. | [optional]
**is_removable** | Option<**bool**> | Whether this payment method can be removed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# CommercePaymentSourceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_source) |
**id** | **String** | Unique identifier for the payment source. |
**payer_id** | **String** | Unique identifier for the payer. |
**payment_method** | **PaymentMethod** | The payment method type. (enum: card, apple_pay, google_pay) |
**is_default** | Option<**bool**> | Whether this is the default payment source for the payer. | [optional]
**gateway** | **String** | The payment gateway. |
**gateway_external_id** | **String** | External ID in the payment gateway. |
**gateway_external_account_id** | Option<**String**> | External account ID in the payment gateway. | [optional]
**last4** | **String** | Last 4 digits of the card (for card payment sources). |
**status** | **Status** | Status of the payment source. (enum: active, disconnected) |
**wallet_type** | **String** | Type of wallet (if applicable). |
**card_type** | **String** | Type of card (if applicable). |
**expiry_year** | Option<**i32**> | Card expiration year (for card payment sources). | [optional]
**expiry_month** | Option<**i32**> | Card expiration month (for card payment sources). | [optional]
**created_at** | **i64** | Unix timestamp (in milliseconds) when the payment source was created. |
**updated_at** | **i64** | Unix timestamp (in milliseconds) when the payment source was last updated. |
**is_removable** | Option<**bool**> | Whether this payment source can be removed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



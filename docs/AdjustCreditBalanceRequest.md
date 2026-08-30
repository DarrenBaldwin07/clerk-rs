# AdjustCreditBalanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **i64** | The credit amount in cents. Must be greater than zero. |
**action** | **Action** | Whether to increase or decrease the credit balance. (enum: increase, decrease) |
**currency** | Option<**String**> | The currency code (e.g. \"USD\"). Defaults to USD if not provided. | [optional]
**idempotency_key** | **String** | A unique key to ensure the adjustment is applied only once. Repeated requests with the same key return the original ledger entry. |
**note** | Option<**String**> | An optional note to attach to the ledger entry. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# CreateBillingPriceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plan_id** | **String** | The ID of the plan this price belongs to. |
**currency** | Option<**String**> | The currency code (e.g., \"USD\"). Defaults to USD. | [optional][default to USD]
**amount** | Option<**i64**> | The monthly amount in cents. Use `0` for a complimentary price. Positive amounts must be at least $1 (100 cents). |
**annual_monthly_amount** | Option<**i64**> | The monthly amount in cents when billed annually. Use `0` for a complimentary price. Positive amounts must be at least $1 (100 cents). | [optional]
**description** | Option<**String**> | An optional description for this custom price. | [optional]
**supported_billing_periods** | Option<**SupportedBillingPeriods**> | Which billing periods this price supports. Inferred from amounts if omitted. (enum: month, annual, both) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# BillingPriceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_price) |
**id** | **String** | Unique identifier for the price. |
**plan_id** | **String** | Unique identifier for the associated plan. |
**instance_id** | **String** | Unique identifier for the instance. |
**currency** | **String** | The currency code (e.g., \"USD\"). |
**currency_symbol** | **String** | The currency symbol (e.g., \"$\"). |
**amount** | **i64** | The amount in cents for the price. |
**annual_monthly_amount** | **i64** | The monthly amount in cents when billed annually. |
**fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**annual_monthly_fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**description** | Option<**String**> | The description of the price. | [optional]
**is_default** | **bool** | Whether this price is the default price for its plan. |
**created_at** | **i64** | Unix timestamp (milliseconds) of creation. |
**supported_billing_periods** | **SupportedBillingPeriods** | Which billing periods this price supports. (enum: month, annual, both) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



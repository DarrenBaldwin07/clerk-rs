# CommercePriceTransitionDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**previous_plan** | [**models::CommercePlan2**](CommercePlan2.md) |  |
**previous_price** | [**models::BillingPriceResponse**](BillingPriceResponse.md) |  |
**effective_at** | **i64** | Unix timestamp (milliseconds) when the new price takes effect. |
**effective_mode** | **EffectiveMode** | When the new price takes effect. (enum: immediate, end_of_period) |
**next_billing_date** | Option<**i64**> | Unix timestamp (milliseconds) for the next billing date. | [optional]
**charged_immediately** | **bool** | Whether an immediate charge was made. |
**immediate_charge** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> | Amount charged immediately, if any. | [optional]
**previous_subscription_item_status** | **PreviousSubscriptionItemStatus** | The status of the previous subscription item after transition. (enum: canceled, ended, abandoned) |
**previous_subscription_item_id** | **String** | The ID of the previous subscription item. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



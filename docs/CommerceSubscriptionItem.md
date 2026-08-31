# CommerceSubscriptionItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_subscription_item) |
**id** | **String** | Unique identifier for the subscription item. |
**instance_id** | **String** | Unique identifier for the Clerk instance. |
**status** | **Status** | Current status of the subscription item. (enum: active, canceled, expired, ended, past_due, upcoming, incomplete, abandoned) |
**credit** | Option<[**models::CommerceSubscriptionCreditResponse**](CommerceSubscriptionCreditResponse.md)> |  | [optional]
**credits** | Option<[**models::CommerceCreditsResponse**](CommerceCreditsResponse.md)> | Unified credits breakdown for this subscription item. | [optional]
**plan_id** | Option<**String**> | Unique identifier for the associated plan. |
**price_id** | Option<**String**> | Unique identifier for the associated price | [optional]
**plan** | Option<[**models::CommercePlan**](CommercePlan.md)> | The associated plan. | [optional]
**plan_period** | **PlanPeriod** | The billing period for this subscription item. (enum: month, annual) |
**payment_method** | Option<[**models::CommercePaymentMethodResponse**](CommercePaymentMethodResponse.md)> |  | [optional]
**lifetime_paid** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  | [optional]
**next_payment** | Option<[**models::CommerceSubscriptionItemNextPaymentResponse**](CommerceSubscriptionItemNextPaymentResponse.md)> | Information about the next payment. | [optional]
**payer_id** | **String** | Unique identifier for the payer. |
**payer** | Option<[**models::CommercePayerResponse**](CommercePayerResponse.md)> |  | [optional]
**is_free_trial** | **bool** | Whether this subscription item includes a free trial. |
**period_start** | **i64** | Unix timestamp (in milliseconds) when the current period started. |
**period_end** | Option<**i64**> | Unix timestamp (in milliseconds) when the current period ends. |
**proration_date** | Option<**String**> | The day the subscription item was prorated from. Only available in some responses. | [optional]
**canceled_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the subscription item was canceled. |
**past_due_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the subscription item became past due. |
**ended_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the subscription item ended. |
**created_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the subscription item was created. | [optional]
**updated_at** | Option<**i64**> | Unix timestamp (in milliseconds) when the subscription item was last updated. | [optional]
**seats** | Option<[**models::CommerceSubscriptionItemSeatsResponse**](CommerceSubscriptionItemSeatsResponse.md)> | Seat quantity for seat-based billing. | [optional]
**totals** | Option<[**models::CommerceTotalsResponse2**](CommerceTotalsResponse2.md)> | Totals for this subscription item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



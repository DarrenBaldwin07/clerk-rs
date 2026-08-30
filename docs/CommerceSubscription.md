# CommerceSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_subscription) |
**id** | **String** | Unique identifier for the commerce subscription. |
**instance_id** | **String** | The ID of the instance this subscription belongs to. |
**status** | **Status** | The current status of the subscription. (enum: active, past_due, canceled, ended, abandoned, incomplete) |
**payer_id** | **String** | The ID of the payer for this subscription. |
**created_at** | **i64** | Unix timestamp (milliseconds) of creation. |
**updated_at** | **i64** | Unix timestamp (milliseconds) of last update. |
**active_at** | Option<**i64**> | Unix timestamp (milliseconds) when the subscription became active. |
**past_due_at** | Option<**i64**> | Unix timestamp (milliseconds) when the subscription became past due. |
**subscription_items** | [**Vec<models::CommerceSubscriptionItem>**](CommerceSubscriptionItem.md) | Array of subscription items in this subscription. |
**next_payment** | Option<[**models::CommerceSubscriptionNextPayment**](CommerceSubscriptionNextPayment.md)> |  | [optional]
**eligible_for_free_trial** | Option<**bool**> | Whether the payer is eligible for a free trial. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



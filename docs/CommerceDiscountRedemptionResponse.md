# CommerceDiscountRedemptionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_discount_redemption) |
**id** | **String** | Unique identifier for the discount redemption. |
**subscription_item_id** | **String** | Unique identifier for the subscription item the discount was applied to. |
**discount_id** | **String** | Unique identifier for the discount that was applied. |
**name** | Option<**String**> | The display name of the discount. | [optional]
**source** | **Source** | How the discount was applied to the subscription item. (enum: promotion, manual, promo_code) |
**promo_code** | Option<**String**> | The promo code used to redeem the discount, when applicable. | [optional]
**effect** | Option<**Effect**> | The snapshotted discount effect. (enum: percentage, fixed_amount) | [optional]
**percent_off** | Option<**f64**> | Percent off when the effect is percentage. | [optional]
**amount_off** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> | Fixed amount off when the effect is fixed_amount. | [optional]
**amount** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> | How much this discount takes off the subscription item's next renewal charge. Present for either effect when next-payment data is available. | [optional]
**cycles_remaining** | Option<**i32**> | Remaining billing cycles the discount applies to. Null means the discount lasts forever. |
**cycles_applied** | **i32** | Number of billing cycles the discount has already been applied to. |
**status** | Option<**Status**> | Current status of the discount redemption. (enum: active, exhausted, removed) | [optional]
**redeemed_at** | **i64** | Unix timestamp (in milliseconds) when the discount was redeemed. |
**redeemed_by** | Option<**String**> | Identifier of the actor that redeemed the discount, when available. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



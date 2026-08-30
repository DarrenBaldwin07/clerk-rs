# CommercePlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_plan) |
**id** | **String** | Unique identifier for the plan. |
**name** | **String** | The name of the plan. |
**fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**annual_monthly_fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**annual_fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**description** | Option<**String**> | The description of the plan. |
**product_id** | **String** | The ID of the product this plan belongs to. |
**is_default** | **bool** | Whether this is the default plan. |
**is_recurring** | **bool** | Whether this is a recurring plan. |
**publicly_visible** | **bool** | Whether this plan is publicly visible. |
**has_base_fee** | **bool** | Whether this plan has a base fee. |
**for_payer_type** | **String** | The payer type this plan is designed for. |
**slug** | **String** | The URL-friendly slug for the plan. |
**avatar_url** | Option<**String**> | The URL of the plan's avatar image. |
**features** | Option<[**Vec<models::FeatureResponse>**](FeatureResponse.md)> | The features included in this plan. | [optional]
**free_trial_enabled** | **bool** | Whether free trial is enabled for this plan. |
**free_trial_days** | Option<**i64**> | Number of free trial days for this plan. |
**unit_prices** | Option<[**Vec<models::CommercePlanUnitPrice>**](CommercePlanUnitPrice.md)> | Per-unit pricing tiers for this plan (for example, seats) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



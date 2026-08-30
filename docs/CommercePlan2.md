# CommercePlan2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_plan) |
**id** | **String** | Unique identifier for the commerce plan. |
**name** | **String** | The name of the commerce plan. |
**fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**annual_monthly_fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**annual_fee** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> |  |
**amount** | **i64** | The amount in cents for the plan. |
**amount_formatted** | **String** | The formatted amount as a string (e.g., \"$49.99\"). |
**annual_monthly_amount** | **i64** | The monthly amount in cents when billed annually. |
**annual_monthly_amount_formatted** | **String** | The formatted annual monthly amount as a string. |
**annual_amount** | **i64** | The total annual amount in cents. |
**annual_amount_formatted** | **String** | The formatted annual amount as a string. |
**currency_symbol** | **String** | The currency symbol (e.g., \"$\"). |
**currency** | **String** | The currency code (e.g., \"USD\"). |
**description** | **String** | The description of the commerce plan. |
**product_id** | **String** | The ID of the product this plan belongs to. |
**is_default** | **bool** | Whether this is the default plan. |
**is_recurring** | **bool** | Whether this is a recurring plan. |
**publicly_visible** | **bool** | Whether this plan is publicly visible. |
**has_base_fee** | **bool** | Whether this plan has a base fee. |
**payer_type** | **Vec<String>** | The types of payers that can use this plan. |
**for_payer_type** | **String** | The payer type this plan is designed for. |
**slug** | **String** | The URL-friendly slug for the plan. |
**avatar_url** | **String** | The URL of the plan's avatar image. |
**period** | Option<**String**> | The billing period for the plan. | [optional]
**interval** | Option<**i64**> | The billing interval. | [optional]
**features** | [**Vec<models::FeatureResponse2>**](FeatureResponse2.md) | The features included in this plan. |
**free_trial_enabled** | Option<**bool**> | Whether free trial is enabled for this plan. | [optional]
**free_trial_days** | Option<**i64**> | Number of free trial days for this plan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



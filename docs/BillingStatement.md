# BillingStatement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_statement) |
**id** | **String** | Unique identifier for the billing statement. |
**instance_id** | **String** | The ID of the instance this statement belongs to. |
**timestamp** | **i64** | Unix timestamp (in milliseconds) when the statement was created. |
**payer** | [**models::CommercePayerResponse**](CommercePayerResponse.md) |  |
**status** | **Status** | The current status of the statement. (enum: open, closed) |
**totals** | [**models::BillingStatementTotals**](BillingStatementTotals.md) |  |
**groups** | [**Vec<models::BillingStatementGroupsInner>**](BillingStatementGroupsInner.md) | Array of statement groups. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



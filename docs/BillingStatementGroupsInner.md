# BillingStatementGroupsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. (enum: commerce_statement_group) |
**timestamp** | **i64** | Unix timestamp (in milliseconds) of the date the group's payment attempts were created |
**items** | [**Vec<models::BillingPaymentAttempt>**](BillingPaymentAttempt.md) | The payment attempts included in the group |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



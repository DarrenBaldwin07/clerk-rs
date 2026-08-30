# CommerceCreditLedgerResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **String** | String representing the object's type. Always \"commerce_credit_ledger\". |
**id** | **String** | Unique identifier for the ledger entry. |
**payer_id** | **String** | The ID of the payer whose balance was adjusted. |
**amount** | [**models::CommerceMoneyResponse**](CommerceMoneyResponse.md) |  |
**source_type** | **String** | The type of source that originated the adjustment (e.g. \"grant\"). |
**source_id** | **String** | The ID of the source that originated the adjustment. |
**note** | Option<**String**> | An optional note attached to the ledger entry. | [optional]
**created_at** | **String** | Timestamp when the ledger entry was created. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



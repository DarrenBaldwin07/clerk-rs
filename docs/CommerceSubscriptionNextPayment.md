# CommerceSubscriptionNextPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | **i64** | Unix timestamp (milliseconds) of the next payment date. |
**amount** | [**models::CommerceMoneyResponse**](CommerceMoneyResponse.md) |  |
**per_unit_totals** | Option<[**Vec<models::CommercePerUnitTotal2>**](CommercePerUnitTotal2.md)> | Per-unit total breakdown (for example, seats) for the next payment. | [optional]
**totals** | Option<[**models::CommerceTotalsResponse2**](CommerceTotalsResponse2.md)> | Breakdown of the recurring amount that will be billed at renewal (base fee + per-unit charges). Tax and credits are not previewed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



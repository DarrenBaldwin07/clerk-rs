# CommerceSubscriptionItemNextPaymentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<[**models::CommerceMoneyResponse**](CommerceMoneyResponse.md)> | Base plan fee for the next payment. Does not include per-unit (e.g. seat) charges; see `totals.grand_total` for the full amount. | [optional]
**date** | Option<**i64**> | Unix timestamp (in milliseconds) for the next payment date. | [optional]
**per_unit_totals** | Option<[**Vec<models::CommercePerUnitTotal>**](CommercePerUnitTotal.md)> | Per-unit total breakdown (for example, seats) for the next payment. | [optional]
**totals** | Option<[**models::CommerceTotalsResponse**](CommerceTotalsResponse.md)> | Breakdown of the recurring amount that will be billed at renewal (base fee + per-unit charges). Tax and credits are not previewed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# \BillingApi

All URIs are relative to *https://api.clerk.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adjust_organization_billing_credit_balance**](BillingApi.md#adjust_organization_billing_credit_balance) | **POST** /organizations/{organization_id}/billing/credits | Adjust an organization's credit balance
[**adjust_user_billing_credit_balance**](BillingApi.md#adjust_user_billing_credit_balance) | **POST** /users/{user_id}/billing/credits | Adjust a user's credit balance
[**apply_billing_subscription_item_discount**](BillingApi.md#apply_billing_subscription_item_discount) | **POST** /billing/subscription_items/{subscription_item_id}/discounts | Apply a discount to a subscription item
[**cancel_commerce_subscription_item**](BillingApi.md#cancel_commerce_subscription_item) | **DELETE** /billing/subscription_items/{subscription_item_id} | Cancel a subscription item
[**create_billing_price**](BillingApi.md#create_billing_price) | **POST** /billing/prices | Create a custom billing price
[**create_billing_price_transition**](BillingApi.md#create_billing_price_transition) | **POST** /billing/subscription_items/{subscription_item_id}/price_transition | Create a price transition for a subscription item
[**extend_billing_subscription_item_free_trial**](BillingApi.md#extend_billing_subscription_item_free_trial) | **POST** /billing/subscription_items/{subscription_item_id}/extend_free_trial | Extend free trial for a subscription item
[**get_billing_price_list**](BillingApi.md#get_billing_price_list) | **GET** /billing/prices | List all billing prices
[**get_billing_statement**](BillingApi.md#get_billing_statement) | **GET** /billing/statements/{statementID} | Retrieve a billing statement
[**get_billing_statement_list**](BillingApi.md#get_billing_statement_list) | **GET** /billing/statements | List all billing statements
[**get_billing_statement_payment_attempts**](BillingApi.md#get_billing_statement_payment_attempts) | **GET** /billing/statements/{statementID}/payment_attempts | List payment attempts for a billing statement
[**get_commerce_plan_list**](BillingApi.md#get_commerce_plan_list) | **GET** /billing/plans | List all billing plans
[**get_commerce_subscription_item_list**](BillingApi.md#get_commerce_subscription_item_list) | **GET** /billing/subscription_items | List all subscription items
[**get_organization_billing_credit_balance**](BillingApi.md#get_organization_billing_credit_balance) | **GET** /organizations/{organization_id}/billing/credits | Retrieve an organization's credit balance
[**get_organization_billing_subscription**](BillingApi.md#get_organization_billing_subscription) | **GET** /organizations/{organization_id}/billing/subscription | Retrieve an organization's billing subscription
[**get_user_billing_credit_balance**](BillingApi.md#get_user_billing_credit_balance) | **GET** /users/{user_id}/billing/credits | Retrieve a user's credit balance
[**get_user_billing_subscription**](BillingApi.md#get_user_billing_subscription) | **GET** /users/{user_id}/billing/subscription | Retrieve a user's billing subscription
[**remove_billing_subscription_item_discount**](BillingApi.md#remove_billing_subscription_item_discount) | **DELETE** /billing/subscription_items/{subscription_item_id}/discounts/{discount_id} | Remove a discount from a subscription item



## adjust_organization_billing_credit_balance

> models::CommerceCreditLedgerResponse adjust_organization_billing_credit_balance(organization_id, adjust_credit_balance_request)
Adjust an organization's credit balance

Increases or decreases the credit balance for the specified organization. Each adjustment is recorded as a ledger entry. The idempotency_key parameter ensures that duplicate requests are safely handled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization whose credit balance to adjust | [required] |
**adjust_credit_balance_request** | [**AdjustCreditBalanceRequest**](AdjustCreditBalanceRequest.md) | Parameters for the credit balance adjustment | [required] |

### Return type

[**models::CommerceCreditLedgerResponse**](CommerceCreditLedgerResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## adjust_user_billing_credit_balance

> models::CommerceCreditLedgerResponse adjust_user_billing_credit_balance(user_id, adjust_credit_balance_request)
Adjust a user's credit balance

Increases or decreases the credit balance for the specified user. Each adjustment is recorded as a ledger entry. The idempotency_key parameter ensures that duplicate requests are safely handled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose credit balance to adjust | [required] |
**adjust_credit_balance_request** | [**AdjustCreditBalanceRequest**](AdjustCreditBalanceRequest.md) | Parameters for the credit balance adjustment | [required] |

### Return type

[**models::CommerceCreditLedgerResponse**](CommerceCreditLedgerResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apply_billing_subscription_item_discount

> models::CommerceDiscountRedemptionResponse apply_billing_subscription_item_discount(subscription_item_id, apply_commerce_discount_request)
Apply a discount to a subscription item

Applies an existing discount to a subscription item. Manual application is an override path: self-serve distribution rules are not enforced. At most one active discount is allowed per subscription item; applying a different discount replaces the currently active one. Re-applying the same active discount returns a conflict.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_item_id** | **String** | The ID of the subscription item to apply the discount to | [required] |
**apply_commerce_discount_request** | [**ApplyCommerceDiscountRequest**](ApplyCommerceDiscountRequest.md) | Parameters for applying the discount | [required] |

### Return type

[**models::CommerceDiscountRedemptionResponse**](CommerceDiscountRedemptionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_commerce_subscription_item

> models::CommerceSubscriptionItem cancel_commerce_subscription_item(subscription_item_id, end_now)
Cancel a subscription item

Cancel a specific subscription item. The subscription item can be canceled immediately or at the end of the current billing period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_item_id** | **String** | The ID of the subscription item to cancel | [required] |
**end_now** | Option<**bool**> | Whether to cancel the subscription immediately (true) or at the end of the current billing period (false, default) |  |[default to false]

### Return type

[**models::CommerceSubscriptionItem**](CommerceSubscriptionItem.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_billing_price

> models::BillingPriceResponse create_billing_price(create_billing_price_request)
Create a custom billing price

Creates a custom price for a billing plan. Custom prices allow you to offer different pricing to specific customers while maintaining the same plan structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_billing_price_request** | [**CreateBillingPriceRequest**](CreateBillingPriceRequest.md) | Parameters for creating a custom price | [required] |

### Return type

[**models::BillingPriceResponse**](BillingPriceResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_billing_price_transition

> models::CommercePriceTransitionResponse create_billing_price_transition(subscription_item_id, price_transition_request)
Create a price transition for a subscription item

Creates a price transition for the specified subscription item. This may create an upcoming subscription item or activate immediately depending on plan and payer rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_item_id** | **String** | The ID of the subscription item to transition | [required] |
**price_transition_request** | [**PriceTransitionRequest**](PriceTransitionRequest.md) | Parameters for the price transition | [required] |

### Return type

[**models::CommercePriceTransitionResponse**](CommercePriceTransitionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extend_billing_subscription_item_free_trial

> models::CommerceSubscriptionItem2 extend_billing_subscription_item_free_trial(subscription_item_id, extend_free_trial_request)
Extend free trial for a subscription item

Extends the free trial period for a specific subscription item to the specified timestamp. The subscription item must be currently in a free trial period, and the plan must support free trials. The timestamp must be in the future and not more than 365 days from the end of the current trial period This operation is idempotent - repeated requests with the same timestamp will not change the trial period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_item_id** | **String** | The ID of the subscription item to extend the free trial for | [required] |
**extend_free_trial_request** | [**ExtendFreeTrialRequest**](ExtendFreeTrialRequest.md) | Parameters for extending the free trial | [required] |

### Return type

[**models::CommerceSubscriptionItem2**](CommerceSubscriptionItem-2.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_billing_price_list

> models::PaginatedBillingPriceResponse get_billing_price_list(paginated, limit, offset, plan_id)
List all billing prices

Returns a list of all prices for the instance. The prices are returned sorted by amount ascending, then by creation date descending. This includes both default and custom prices. Pagination is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**plan_id** | Option<**String**> | Filter prices by plan ID |  |

### Return type

[**models::PaginatedBillingPriceResponse**](PaginatedBillingPriceResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_billing_statement

> models::BillingStatement get_billing_statement(statement_id)
Retrieve a billing statement

Retrieves the details of a billing statement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**statement_id** | **String** | The ID of the statement to retrieve. | [required] |

### Return type

[**models::BillingStatement**](BillingStatement.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_billing_statement_list

> models::PaginatedBillingStatementResponse get_billing_statement_list(paginated, limit, offset)
List all billing statements

Returns a list of all billing statements for the instance. The statements are returned sorted by creation date, with the newest statements appearing first. Pagination is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::PaginatedBillingStatementResponse**](PaginatedBillingStatementResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_billing_statement_payment_attempts

> models::PaginatedBillingPaymentAttemptResponse get_billing_statement_payment_attempts(statement_id, paginated, limit, offset)
List payment attempts for a billing statement

Returns a list of all payment attempts for a specific billing statement. The payment attempts are returned sorted by creation date, with the newest payment attempts appearing first. Pagination is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**statement_id** | **String** | The ID of the statement to retrieve payment attempts for. | [required] |
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]

### Return type

[**models::PaginatedBillingPaymentAttemptResponse**](PaginatedBillingPaymentAttemptResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_commerce_plan_list

> models::PaginatedCommercePlanResponse get_commerce_plan_list(paginated, limit, offset, payer_type)
List all billing plans

Returns a list of all billing plans for the instance. The plans are returned sorted by creation date, with the newest plans appearing first. This includes both free and paid plans. Pagination is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**payer_type** | Option<**String**> | Filter plans by payer type |  |

### Return type

[**models::PaginatedCommercePlanResponse**](PaginatedCommercePlanResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_commerce_subscription_item_list

> models::PaginatedCommerceSubscriptionItemResponse get_commerce_subscription_item_list(paginated, limit, offset, status, payer_type, plan_id, include_free, query)
List all subscription items

Returns a list of all subscription items for the instance. The subscription items are returned sorted by creation date, with the newest appearing first. This includes subscriptions for both users and organizations. Pagination is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**paginated** | Option<**bool**> | Whether to paginate the results. If true, the results will be paginated. If false, the results will not be paginated. |  |
**limit** | Option<**u32**> | Applies a limit to the number of results returned. Can be used for paginating the results together with `offset`. |  |[default to 10]
**offset** | Option<**u32**> | Skip the first `offset` results when paginating. Needs to be an integer greater or equal to zero. To be used in conjunction with `limit`. |  |[default to 0]
**status** | Option<**String**> | Filter subscription items by status |  |
**payer_type** | Option<**String**> | Filter subscription items by payer type |  |
**plan_id** | Option<**String**> | Filter subscription items by plan ID |  |
**include_free** | Option<**bool**> | Whether to include free plan subscription items |  |[default to false]
**query** | Option<**String**> | Search query to filter subscription items |  |

### Return type

[**models::PaginatedCommerceSubscriptionItemResponse**](PaginatedCommerceSubscriptionItemResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_billing_credit_balance

> models::CommerceCreditBalanceResponse get_organization_billing_credit_balance(organization_id)
Retrieve an organization's credit balance

Retrieves the current credit balance for the specified organization. Credits can be applied during checkout to reduce the charge or automatically applied to upcoming recurring charges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization whose credit balance to retrieve | [required] |

### Return type

[**models::CommerceCreditBalanceResponse**](CommerceCreditBalanceResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_billing_subscription

> models::CommerceSubscription get_organization_billing_subscription(organization_id)
Retrieve an organization's billing subscription

Retrieves the billing subscription for the specified organization. This includes subscription details, active plans, billing information, and payment status. The subscription contains subscription items which represent the individual plans the organization is subscribed to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** | The ID of the organization whose subscription to retrieve | [required] |

### Return type

[**models::CommerceSubscription**](CommerceSubscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_billing_credit_balance

> models::CommerceCreditBalanceResponse get_user_billing_credit_balance(user_id)
Retrieve a user's credit balance

Retrieves the current credit balance for the specified user. Credits can be applied during checkout to reduce the charge or automatically applied to upcoming recurring charges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose credit balance to retrieve | [required] |

### Return type

[**models::CommerceCreditBalanceResponse**](CommerceCreditBalanceResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_billing_subscription

> models::CommerceSubscription get_user_billing_subscription(user_id)
Retrieve a user's billing subscription

Retrieves the billing subscription for the specified user. This includes subscription details, active plans, billing information, and payment status. The subscription contains subscription items which represent the individual plans the user is subscribed to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user whose subscription to retrieve | [required] |

### Return type

[**models::CommerceSubscription**](CommerceSubscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_billing_subscription_item_discount

> models::CommerceDiscountRedemptionResponse remove_billing_subscription_item_discount(subscription_item_id, discount_id)
Remove a discount from a subscription item

Removes the active discount from a subscription item. The discount_id must match the subscription item's currently active discount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_item_id** | **String** | The ID of the subscription item to remove the discount from | [required] |
**discount_id** | **String** | The ID of the discount to remove | [required] |

### Return type

[**models::CommerceDiscountRedemptionResponse**](CommerceDiscountRedemptionResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


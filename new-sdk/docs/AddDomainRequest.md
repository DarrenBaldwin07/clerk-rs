# AddDomainRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The new domain name. Can contain the port for development instances. | 
**is_satellite** | **bool** | Marks the new domain as satellite. Only `true` is accepted at the moment. | 
**proxy_url** | Option<**String**> | The full URL of the proxy which will forward requests to the Clerk Frontend API for this domain. Applicable only to production instances. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



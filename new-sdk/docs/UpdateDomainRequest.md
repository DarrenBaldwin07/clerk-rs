# UpdateDomainRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The new domain name. For development instances, can contain the port, i.e `myhostname:3000`. For production instances, must be a valid FQDN, i.e `mysite.com`. Cannot contain protocol scheme. | [optional]
**proxy_url** | Option<**String**> | The full URL of the proxy that will forward requests to Clerk's Frontend API. Can only be updated for production instances. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



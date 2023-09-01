# UpdateSamlConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the new SAML Connection | [optional]
**domain** | Option<**String**> | The domain to use for the new SAML Connection | [optional]
**idp_entity_id** | Option<**String**> | The entity id as provided by the IdP | [optional]
**idp_sso_url** | Option<**String**> | The SSO url as provided by the IdP | [optional]
**idp_certificate** | Option<**String**> | The x509 certificated as provided by the IdP | [optional]
**active** | Option<**bool**> | Activate or de-activate the SAML Connection | [optional]
**sync_user_attributes** | Option<**bool**> | Controls whether to update the user's attributes in each sign-in | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ScimDirectory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Always \"scim_directory\". (enum: scim_directory) |
**id** | **String** | Unique identifier for the directory. |
**name** | **String** | A human-friendly name for the directory. |
**enterprise_connection_id** | Option<**String**> | The ID of the associated enterprise connection. | [optional]
**endpoint_url** | **String** | The SCIM endpoint URL for this directory. |
**provider** | **String** | The identity provider for this directory. |
**enabled** | **bool** | Whether the directory is enabled. |
**group_role_mapping_enabled** | **bool** | Whether group-to-role mapping is enabled for this directory. |
**attribute_mapping** | **std::collections::HashMap<String, String>** | Mapping of user attributes to the SCIM attribute paths they are extracted from. |
**custom_attributes** | Option<[**Vec<models::ScimDirectoryCustomAttributesInner>**](SCIMDirectoryCustomAttributesInner.md)> | Custom attributes to map from the IdP to the user's profile via SCIM provisioning. Requires the custom attributes feature to be enabled for the instance. | [optional]
**api_key** | Option<**String**> | The API key for authenticating SCIM requests. Only returned when the directory is created or the key is rotated. | [optional]
**created_at** | **i64** | Unix timestamp when the directory was created. |
**updated_at** | **i64** | Unix timestamp when the directory was last updated. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



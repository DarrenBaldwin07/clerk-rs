# UpdateScimDirectoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A human-friendly name for the directory. | [optional]
**enabled** | Option<**bool**> | Whether the directory is enabled. | [optional]
**provider** | Option<**String**> | The identity provider for this directory. | [optional]
**attribute_mapping** | Option<**std::collections::HashMap<String, String>**> | Attribute-to-SCIM-path entries to merge into the directory's attribute mapping. Set a key to `null` to remove it from the mapping. | [optional]
**group_role_mapping_enabled** | Option<**bool**> | Whether group-to-role mapping is enabled for this directory. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



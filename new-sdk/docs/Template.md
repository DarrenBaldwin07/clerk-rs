# Template

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**object** | Option<**String**> | String representing the object's type. Objects of the same type share the same value.  | [optional]
**instance_id** | Option<**String**> | the id of the instance the template belongs to | [optional]
**resource_type** | Option<**String**> | whether this is a system (default) or user overridden) template | [optional]
**template_type** | Option<**String**> | whether this is an email or SMS template | [optional]
**name** | Option<**String**> | user-friendly name of the template | [optional]
**slug** | Option<**String**> | machine-friendly name of the template | [optional]
**position** | Option<**i32**> | position with the listing of templates | [optional]
**can_revert** | Option<**bool**> | whether this template can be reverted to the corresponding system default | [optional]
**can_delete** | Option<**bool**> | whether this template can be deleted | [optional]
**subject** | Option<**String**> | email subject | [optional]
**markup** | Option<**String**> | the editor markup used to generate the body of the template | [optional]
**body** | Option<**String**> | the template body before variable interpolation | [optional]
**available_variables** | Option<**Vec<String>**> | list of variables that are available for use in the template body | [optional]
**required_variables** | Option<**Vec<String>**> | list of variables that must be contained in the template body | [optional]
**from_email_name** | Option<**String**> |  | [optional]
**delivered_by_clerk** | Option<**bool**> |  | [optional]
**updated_at** | Option<**i64**> | Unix timestamp of last update.  | [optional]
**created_at** | Option<**i64**> | Unix timestamp of creation.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



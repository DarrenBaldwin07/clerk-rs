# OrganizationDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | **Object** | String representing the object's type. Objects of the same type share the same value. Always `organization_domain`  (enum: organization_domain) |
**id** | **String** | Unique identifier for the organization domain |
**organization_id** | **String** | Unique identifier for the organization |
**name** | **String** | Name of the organization domain |
**enrollment_mode** | **EnrollmentMode** | Mode of enrollment for the domain (enum: manual_invitation, automatic_invitation, automatic_suggestion, enterprise_sso) |
**affiliation_email_address** | Option<**String**> | Affiliation email address for the domain, if available. |
**affiliation_verification** | Option<[**models::OrganizationDomainVerification**](OrganizationDomainVerification.md)> | Verification details for the user-facing affiliation between the domain and the organization (e.g. affiliation_email_code).  |
**ownership_verification** | Option<[**models::OrganizationDomainVerification**](OrganizationDomainVerification.md)> | Verification details for the underlying DNS domain ownership proof (TXT challenge or dashboard override). Null until ownership has been attempted.  |
**verification** | Option<[**models::OrganizationDomainVerification**](OrganizationDomainVerification.md)> | Deprecated alias for `affiliation_verification`. Kept for backward compatibility on the current API version; will be removed in the next API version. Prefer `affiliation_verification`.  |
**total_pending_invitations** | **i32** | Total number of pending invitations associated with this domain |
**total_pending_suggestions** | **i32** | Total number of pending suggestions associated with this domain |
**public_organization_data** | Option<[**models::OrganizationInvitationPublicOrganizationData**](OrganizationInvitationPublicOrganizationData.md)> | Public organization data associated with this domain | [optional]
**created_at** | **i64** | Unix timestamp when the domain was created |
**updated_at** | **i64** | Unix timestamp of the last update to the domain |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



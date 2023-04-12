## Documentation for API Endpoints

All URIs are relative to *https://api.clerk.dev/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ActorTokensApi* | [**create_actor_token**](docs/ActorTokensApi.md#create_actor_token) | **POST** /actor_tokens | Create actor token
*ActorTokensApi* | [**revoke_actor_token**](docs/ActorTokensApi.md#revoke_actor_token) | **POST** /actor_tokens/{actor_token_id}/revoke | Revoke actor token
*AllowListBlockListApi* | [**create_allowlist_identifier**](docs/AllowListBlockListApi.md#create_allowlist_identifier) | **POST** /allowlist_identifiers | Add identifier to the allow-list
*AllowListBlockListApi* | [**create_blocklist_identifier**](docs/AllowListBlockListApi.md#create_blocklist_identifier) | **POST** /blocklist_identifiers | Add identifier to the block-list
*AllowListBlockListApi* | [**delete_allowlist_identifier**](docs/AllowListBlockListApi.md#delete_allowlist_identifier) | **DELETE** /allowlist_identifiers/{identifier_id} | Delete identifier from allow-list
*AllowListBlockListApi* | [**delete_blocklist_identifier**](docs/AllowListBlockListApi.md#delete_blocklist_identifier) | **DELETE** /blocklist_identifiers/{identifier_id} | Delete identifier from block-list
*AllowListBlockListApi* | [**list_allowlist_identifiers**](docs/AllowListBlockListApi.md#list_allowlist_identifiers) | **GET** /allowlist_identifiers | List all identifiers on the allow-list
*AllowListBlockListApi* | [**list_blocklist_identifiers**](docs/AllowListBlockListApi.md#list_blocklist_identifiers) | **GET** /blocklist_identifiers | List all identifiers on the block-list
*BetaFeaturesApi* | [**update_instance_auth_config**](docs/BetaFeaturesApi.md#update_instance_auth_config) | **PATCH** /beta_features/instance_settings | Update instance settings
*BetaFeaturesApi* | [**update_production_instance_domain**](docs/BetaFeaturesApi.md#update_production_instance_domain) | **PUT** /beta_features/domain | Update production instance domain
*ClientsApi* | [**get_client**](docs/ClientsApi.md#get_client) | **GET** /clients/{client_id} | Get a client
*ClientsApi* | [**get_client_last_active_session**](docs/ClientsApi.md#get_client_last_active_session) | **GET** /clients/{client_id}/last_active_session | Get the last active session of a client
*ClientsApi* | [**get_client_list**](docs/ClientsApi.md#get_client_list) | **GET** /clients | List all clients
*ClientsApi* | [**verify_client**](docs/ClientsApi.md#verify_client) | **POST** /clients/verify | Verify a client
*EmailAddressesApi* | [**create_email_address**](docs/EmailAddressesApi.md#create_email_address) | **POST** /email_addresses | Create an email address
*EmailAddressesApi* | [**delete_email_address**](docs/EmailAddressesApi.md#delete_email_address) | **DELETE** /email_addresses/{email_address_id} | Delete an email address
*EmailAddressesApi* | [**get_email_address**](docs/EmailAddressesApi.md#get_email_address) | **GET** /email_addresses/{email_address_id} | Retrieve an email address
*EmailAddressesApi* | [**update_email_address**](docs/EmailAddressesApi.md#update_email_address) | **PATCH** /email_addresses/{email_address_id} | Update an email address
*EmailSmsTemplatesApi* | [**get_template**](docs/EmailSmsTemplatesApi.md#get_template) | **GET** /templates/{template_type}/{slug} | Retrieve a template
*EmailSmsTemplatesApi* | [**get_template_list**](docs/EmailSmsTemplatesApi.md#get_template_list) | **GET** /templates/{template_type} | List all templates
*EmailSmsTemplatesApi* | [**preview_template**](docs/EmailSmsTemplatesApi.md#preview_template) | **POST** /templates/{template_type}/{slug}/preview | Preview changes to a template
*EmailSmsTemplatesApi* | [**revert_template**](docs/EmailSmsTemplatesApi.md#revert_template) | **POST** /templates/{template_type}/{slug}/revert | Revert a template
*EmailSmsTemplatesApi* | [**upsert_template**](docs/EmailSmsTemplatesApi.md#upsert_template) | **PUT** /templates/{template_type}/{slug} | Update a template for a given type and slug
*EmailsApi* | [**create_email**](docs/EmailsApi.md#create_email) | **POST** /emails | Create an email
*InstanceSettingsApi* | [**update_instance**](docs/InstanceSettingsApi.md#update_instance) | **PATCH** /instance | Update instance settings
*InstanceSettingsApi* | [**update_instance_organization_settings**](docs/InstanceSettingsApi.md#update_instance_organization_settings) | **PATCH** /instance/organization_settings | Update instance organization settings
*InstanceSettingsApi* | [**update_instance_restrictions**](docs/InstanceSettingsApi.md#update_instance_restrictions) | **PATCH** /instance/restrictions | Update instance restrictions
*InvitationsApi* | [**create_invitation**](docs/InvitationsApi.md#create_invitation) | **POST** /invitations | Create an invitation
*InvitationsApi* | [**list_invitations**](docs/InvitationsApi.md#list_invitations) | **GET** /invitations | List all invitations
*InvitationsApi* | [**revoke_invitation**](docs/InvitationsApi.md#revoke_invitation) | **POST** /invitations/{invitation_id}/revoke | Revokes an invitation
*JwksApi* | [**get_jwks**](docs/JwksApi.md#get_jwks) | **GET** /jwks | Retrieve the JSON Web Key Set of the instance
*JwtTemplatesApi* | [**create_jwt_template**](docs/JwtTemplatesApi.md#create_jwt_template) | **POST** /jwt_templates | Create a JWT template
*JwtTemplatesApi* | [**delete_jwt_template**](docs/JwtTemplatesApi.md#delete_jwt_template) | **DELETE** /jwt_templates/{template_id} | Delete a Template
*JwtTemplatesApi* | [**get_jwt_template**](docs/JwtTemplatesApi.md#get_jwt_template) | **GET** /jwt_templates/{template_id} | Retrieve a template
*JwtTemplatesApi* | [**list_jwt_templates**](docs/JwtTemplatesApi.md#list_jwt_templates) | **GET** /jwt_templates | List all templates
*JwtTemplatesApi* | [**update_jwt_template**](docs/JwtTemplatesApi.md#update_jwt_template) | **PATCH** /jwt_templates/{template_id} | Update a JWT template
*MiscellaneousApi* | [**create_demo_instance**](docs/MiscellaneousApi.md#create_demo_instance) | **POST** /public/demo_instance | Create a demo development instance
*MiscellaneousApi* | [**get_public_interstitial**](docs/MiscellaneousApi.md#get_public_interstitial) | **GET** /public/interstitial | Returns the markup for the interstitial page
*OrganizationInvitationsApi* | [**create_organization_invitation**](docs/OrganizationInvitationsApi.md#create_organization_invitation) | **POST** /organizations/{organization_id}/invitations | Create and send an organization invitation
*OrganizationInvitationsApi* | [**list_pending_organization_invitations**](docs/OrganizationInvitationsApi.md#list_pending_organization_invitations) | **GET** /organizations/{organization_id}/invitations/pending | Get a list of pending organization invitations
*OrganizationInvitationsApi* | [**revoke_organization_invitation**](docs/OrganizationInvitationsApi.md#revoke_organization_invitation) | **POST** /organizations/{organization_id}/invitations/{invitation_id}/revoke | Revoke a pending organization invitation
*OrganizationMembershipsApi* | [**create_organization_membership**](docs/OrganizationMembershipsApi.md#create_organization_membership) | **POST** /organizations/{organization_id}/memberships | Create a new organization membership
*OrganizationMembershipsApi* | [**delete_organization_membership**](docs/OrganizationMembershipsApi.md#delete_organization_membership) | **DELETE** /organizations/{organization_id}/memberships/{user_id} | Remove a member from an organization
*OrganizationMembershipsApi* | [**list_organization_memberships**](docs/OrganizationMembershipsApi.md#list_organization_memberships) | **GET** /organizations/{organization_id}/memberships | Get a list of all members of an organization
*OrganizationMembershipsApi* | [**update_organization_membership**](docs/OrganizationMembershipsApi.md#update_organization_membership) | **PATCH** /organizations/{organization_id}/memberships/{user_id} | Update an organization membership
*OrganizationMembershipsApi* | [**update_organization_membership_metadata**](docs/OrganizationMembershipsApi.md#update_organization_membership_metadata) | **PATCH** /organizations/{organization_id}/memberships/{user_id}/metadata | Merge and update organization membership metadata
*OrganizationsApi* | [**create_organization**](docs/OrganizationsApi.md#create_organization) | **POST** /organizations | Create an organization
*OrganizationsApi* | [**delete_organization**](docs/OrganizationsApi.md#delete_organization) | **DELETE** /organizations/{organization_id} | Delete an organization
*OrganizationsApi* | [**get_organization**](docs/OrganizationsApi.md#get_organization) | **GET** /organizations/{organization_id} | Retrieve an organization by ID or slug
*OrganizationsApi* | [**list_organizations**](docs/OrganizationsApi.md#list_organizations) | **GET** /organizations | Get a list of organizations for an instance
*OrganizationsApi* | [**merge_organization_metadata**](docs/OrganizationsApi.md#merge_organization_metadata) | **PATCH** /organizations/{organization_id}/metadata | Merge and update metadata for an organization
*OrganizationsApi* | [**update_organization**](docs/OrganizationsApi.md#update_organization) | **PATCH** /organizations/{organization_id} | Update an organization
*OrganizationsApi* | [**upload_organization_logo**](docs/OrganizationsApi.md#upload_organization_logo) | **PUT** /organizations/{organization_id}/logo | Upload a logo for the organization
*PhoneNumbersApi* | [**create_phone_number**](docs/PhoneNumbersApi.md#create_phone_number) | **POST** /phone_numbers | Create a phone number
*PhoneNumbersApi* | [**delete_phone_number**](docs/PhoneNumbersApi.md#delete_phone_number) | **DELETE** /phone_numbers/{phone_number_id} | Delete a phone number
*PhoneNumbersApi* | [**get_phone_number**](docs/PhoneNumbersApi.md#get_phone_number) | **GET** /phone_numbers/{phone_number_id} | Retrieve a phone number
*PhoneNumbersApi* | [**update_phone_number**](docs/PhoneNumbersApi.md#update_phone_number) | **PATCH** /phone_numbers/{phone_number_id} | Update a phone number
*RedirectUrlsApi* | [**create_redirect_url**](docs/RedirectUrlsApi.md#create_redirect_url) | **POST** /redirect_urls |
*RedirectUrlsApi* | [**delete_redirect_url**](docs/RedirectUrlsApi.md#delete_redirect_url) | **DELETE** /redirect_urls/{id} | Delete a redirect URL
*RedirectUrlsApi* | [**get_redirect_url**](docs/RedirectUrlsApi.md#get_redirect_url) | **GET** /redirect_urls/{id} | Retrieve a redirect URL
*RedirectUrlsApi* | [**list_redirect_urls**](docs/RedirectUrlsApi.md#list_redirect_urls) | **GET** /redirect_urls | List all redirect URLs
*SmsMessagesApi* | [**create_sms_message**](docs/SmsMessagesApi.md#create_sms_message) | **POST** /sms_messages | Create an SMS message
*SessionsApi* | [**create_session_token_from_template**](docs/SessionsApi.md#create_session_token_from_template) | **POST** /sessions/{session_id}/tokens/{template_name} | Create a session token from a jwt template
*SessionsApi* | [**get_session**](docs/SessionsApi.md#get_session) | **GET** /sessions/{session_id} | Retrieve a session
*SessionsApi* | [**get_session_list**](docs/SessionsApi.md#get_session_list) | **GET** /sessions | List all sessions
*SessionsApi* | [**revoke_session**](docs/SessionsApi.md#revoke_session) | **POST** /sessions/{session_id}/revoke | Revoke a session
*SessionsApi* | [**verify_session**](docs/SessionsApi.md#verify_session) | **POST** /sessions/{session_id}/verify | Verify a session
*SignInTokensApi* | [**create_sign_in_token**](docs/SignInTokensApi.md#create_sign_in_token) | **POST** /sign_in_tokens | Create sign-in token
*SignInTokensApi* | [**revoke_sign_in_token**](docs/SignInTokensApi.md#revoke_sign_in_token) | **POST** /sign_in_tokens/{sign_in_token_id}/revoke | Revoke the given sign-in token
*SignUpsApi* | [**update_sign_up**](docs/SignUpsApi.md#update_sign_up) | **PATCH** /sign_ups/{id} | Update a sign-up
*UsersApi* | [**ban_user**](docs/UsersApi.md#ban_user) | **POST** /users/{user_id}/ban | Ban a user
*UsersApi* | [**create_user**](docs/UsersApi.md#create_user) | **POST** /users | Create a new user
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /users/{user_id} | Delete a user
*UsersApi* | [**disable_mfa**](docs/UsersApi.md#disable_mfa) | **DELETE** /users/{user_id}/mfa | Disable a user's MFA methods
*UsersApi* | [**get_o_auth_access_token**](docs/UsersApi.md#get_o_auth_access_token) | **GET** /users/{user_id}/oauth_access_tokens/{provider} | Retrieve the OAuth access token of a user
*UsersApi* | [**get_user**](docs/UsersApi.md#get_user) | **GET** /users/{user_id} | Retrieve a user
*UsersApi* | [**get_user_list**](docs/UsersApi.md#get_user_list) | **GET** /users | List all users
*UsersApi* | [**get_users_count**](docs/UsersApi.md#get_users_count) | **GET** /users/count | Count users
*UsersApi* | [**unban_user**](docs/UsersApi.md#unban_user) | **POST** /users/{user_id}/unban | Unban a user
*UsersApi* | [**update_user**](docs/UsersApi.md#update_user) | **PATCH** /users/{user_id} | Update a user
*UsersApi* | [**update_user_metadata**](docs/UsersApi.md#update_user_metadata) | **PATCH** /users/{user_id}/metadata | Merge and update a user's metadata
*UsersApi* | [**users_get_organization_memberships**](docs/UsersApi.md#users_get_organization_memberships) | **GET** /users/{user_id}/organization_memberships | Retrieve all memberships for a user
*UsersApi* | [**verify_password**](docs/UsersApi.md#verify_password) | **POST** /users/{user_id}/verify_password | Verify the password of a user
*UsersApi* | [**verify_totp**](docs/UsersApi.md#verify_totp) | **POST** /users/{user_id}/verify_totp | Verify a TOTP or backup code for a user
*WebhooksApi* | [**create_svix_app**](docs/WebhooksApi.md#create_svix_app) | **POST** /webhooks/svix | Create a Svix app
*WebhooksApi* | [**delete_svix_app**](docs/WebhooksApi.md#delete_svix_app) | **DELETE** /webhooks/svix | Delete a Svix app
*WebhooksApi* | [**generate_svix_auth_url**](docs/WebhooksApi.md#generate_svix_auth_url) | **POST** /webhooks/svix_url | Create a Svix Dashboard URL


## Documentation For Models

 - [ActorToken](docs/ActorToken.md)
 - [Admin](docs/Admin.md)
 - [AllowlistIdentifier](docs/AllowlistIdentifier.md)
 - [BlocklistIdentifier](docs/BlocklistIdentifier.md)
 - [BlocklistIdentifiers](docs/BlocklistIdentifiers.md)
 - [ClerkError](docs/ClerkError.md)
 - [ClerkErrors](docs/ClerkErrors.md)
 - [Client](docs/Client.md)
 - [CreateActorTokenRequest](docs/CreateActorTokenRequest.md)
 - [CreateDemoInstance200Response](docs/CreateDemoInstance200Response.md)
 - [CreateEmailAddressRequest](docs/CreateEmailAddressRequest.md)
 - [CreateEmailRequest](docs/CreateEmailRequest.md)
 - [CreateInvitationRequest](docs/CreateInvitationRequest.md)
 - [CreateJwtTemplateRequest](docs/CreateJwtTemplateRequest.md)
 - [CreateOrganizationInvitationRequest](docs/CreateOrganizationInvitationRequest.md)
 - [CreateOrganizationMembershipRequest](docs/CreateOrganizationMembershipRequest.md)
 - [CreateOrganizationRequest](docs/CreateOrganizationRequest.md)
 - [CreatePhoneNumberRequest](docs/CreatePhoneNumberRequest.md)
 - [CreateRedirectUrlRequest](docs/CreateRedirectUrlRequest.md)
 - [CreateSessionTokenFromTemplate200Response](docs/CreateSessionTokenFromTemplate200Response.md)
 - [CreateSignInTokenRequest](docs/CreateSignInTokenRequest.md)
 - [CreateSmsMessageRequest](docs/CreateSmsMessageRequest.md)
 - [CreateUserRequest](docs/CreateUserRequest.md)
 - [DeletedObject](docs/DeletedObject.md)
 - [DisableMfa200Response](docs/DisableMfa200Response.md)
 - [Email](docs/Email.md)
 - [EmailAddress](docs/EmailAddress.md)
 - [EmailAddressVerification](docs/EmailAddressVerification.md)
 - [GetOAuthAccessToken200ResponseInner](docs/GetOAuthAccessToken200ResponseInner.md)
 - [IdentificationLink](docs/IdentificationLink.md)
 - [InstanceRestrictions](docs/InstanceRestrictions.md)
 - [Invitation](docs/Invitation.md)
 - [JwtTemplate](docs/JwtTemplate.md)
 - [MergeOrganizationMetadataRequest](docs/MergeOrganizationMetadataRequest.md)
 - [Organization](docs/Organization.md)
 - [OrganizationInvitation](docs/OrganizationInvitation.md)
 - [OrganizationInvitations](docs/OrganizationInvitations.md)
 - [OrganizationMembership](docs/OrganizationMembership.md)
 - [OrganizationMembershipOrganization](docs/OrganizationMembershipOrganization.md)
 - [OrganizationMembershipPublicUserData](docs/OrganizationMembershipPublicUserData.md)
 - [OrganizationMemberships](docs/OrganizationMemberships.md)
 - [OrganizationSettings](docs/OrganizationSettings.md)
 - [OrganizationWithLogo](docs/OrganizationWithLogo.md)
 - [OrganizationWithLogoAllOf](docs/OrganizationWithLogoAllOf.md)
 - [Organizations](docs/Organizations.md)
 - [Otp](docs/Otp.md)
 - [PhoneNumber](docs/PhoneNumber.md)
 - [PreviewTemplateRequest](docs/PreviewTemplateRequest.md)
 - [RedirectUrl](docs/RedirectUrl.md)
 - [RevokeOrganizationInvitationRequest](docs/RevokeOrganizationInvitationRequest.md)
 - [Session](docs/Session.md)
 - [SignInToken](docs/SignInToken.md)
 - [SignUp](docs/SignUp.md)
 - [SmsMessage](docs/SmsMessage.md)
 - [SvixUrl](docs/SvixUrl.md)
 - [Template](docs/Template.md)
 - [TotalCount](docs/TotalCount.md)
 - [UpdateEmailAddressRequest](docs/UpdateEmailAddressRequest.md)
 - [UpdateInstanceAuthConfig200Response](docs/UpdateInstanceAuthConfig200Response.md)
 - [UpdateInstanceAuthConfigRequest](docs/UpdateInstanceAuthConfigRequest.md)
 - [UpdateInstanceOrganizationSettingsRequest](docs/UpdateInstanceOrganizationSettingsRequest.md)
 - [UpdateInstanceRequest](docs/UpdateInstanceRequest.md)
 - [UpdateInstanceRestrictionsRequest](docs/UpdateInstanceRestrictionsRequest.md)
 - [UpdateOrganizationMembershipMetadataRequest](docs/UpdateOrganizationMembershipMetadataRequest.md)
 - [UpdateOrganizationMembershipRequest](docs/UpdateOrganizationMembershipRequest.md)
 - [UpdateOrganizationRequest](docs/UpdateOrganizationRequest.md)
 - [UpdatePhoneNumberRequest](docs/UpdatePhoneNumberRequest.md)
 - [UpdateProductionInstanceDomainRequest](docs/UpdateProductionInstanceDomainRequest.md)
 - [UpdateSignUpRequest](docs/UpdateSignUpRequest.md)
 - [UpdateUserMetadataRequest](docs/UpdateUserMetadataRequest.md)
 - [UpdateUserRequest](docs/UpdateUserRequest.md)
 - [UpsertTemplateRequest](docs/UpsertTemplateRequest.md)
 - [User](docs/User.md)
 - [VerifyClientRequest](docs/VerifyClientRequest.md)
 - [VerifyPassword200Response](docs/VerifyPassword200Response.md)
 - [VerifyPasswordRequest](docs/VerifyPasswordRequest.md)
 - [VerifySessionRequest](docs/VerifySessionRequest.md)
 - [VerifyTotp200Response](docs/VerifyTotp200Response.md)
 - [VerifyTotpRequest](docs/VerifyTotpRequest.md)
 - [Web3Signature](docs/Web3Signature.md)
 - [Web3Wallet](docs/Web3Wallet.md)
 - [Web3WalletVerification](docs/Web3WalletVerification.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

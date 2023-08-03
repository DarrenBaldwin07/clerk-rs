use std::fmt;

#[derive(Debug)]
pub enum ClerkEndpoint {
	Get(ClerkGetEndpoint),
	Post(ClerkPostEndpoint),
	Delete(ClerkDeleteEndpoint),
	Put(ClerkPutEndpoint),
	Patch(ClerkPatchEndpoint),
	DynamicGet(ClerkDynamicGetEndpoint),
	DynamicPost(ClerkDynamicPostEndpoint),
	DynamicDelete(ClerkDynamicDeleteEndpoint),
	DynamicPut(ClerkDynamicPutEndpoint),
	DynamicPatch(ClerkDynamicPatchEndpoint),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkDynamicGetEndpoint {
	GetClient,
	GetClientLastActiveSession,
	GetEmailAddress,
	GetTemplate,
	GetTemplateList,
	GetJwks,
	GetJwtTemplate,
	ListPendingOrganizationInvitations,
	ListOrganizationMemberships,
	GetOrganization,
	GetPhoneNumber,
	GetRedirectUrl,
	GetSession,
	GetOAuthAccessToken,
	GetUser,
	UsersGetOrganizationMemberships,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkDynamicPostEndpoint {
	RevokeActorToken,
	DeleteBlocklistIdentifier,
	PreviewTemplate,
	RevertTemplate,
	RevokeInvitation,
	CreateOrganizationInvitation,
	RevokeOrganizationInvitation,
	CreateOrganizationMembership,
	CreateSessionTokenFromTemplate,
	RevokeSession,
	VerifySession,
	RevokeSignInToken,
	BanUser,
	UnbanUser,
	VerifyPassword,
	VerifyTotp,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkDynamicDeleteEndpoint {
	DeleteAllowlistIdentifier,
	DeleteEmailAddress,
	DeleteJwtTemplate,
	DeleteOrganizationMembership,
	DeleteOrganization,
	DeletePhoneNumber,
	DeleteRedirectUrl,
	DeleteUser,
	DisableMfa,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkDynamicPutEndpoint {
	UpsertTemplate,
	UploadOrganizationLogo,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkDynamicPatchEndpoint {
	UpdateEmailAddress,
	UpdateJwtTemplate,
	UpdateOrganizationMembershipMetadata,
	MergeOrganizationMetadata,
	UpdateOrganizationMembership,
	UpdateOrganization,
	UpdatePhoneNumber,
	UpdateSignUp,
	UpdateUser,
	UpdateUserMetadata,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkGetEndpoint {
	ListAllowlistIdentifiers,
	ListBlocklistIdentifiers,
	GetClientList,
	ListInvitations,
	ListJwtTemplates,
	GetPublicInterstitial,
	ListOrganizations,
	ListRedirectUrls,
	GetSessionList,
	GetUserList,
	GetUsersCount,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkPostEndpoint {
	CreateActorToken,
	CreateAllowlistIdentifier,
	CreateBlocklistIdentifier,
	VerifyClient,
	CreateEmailAddress,
	CreateEmail,
	CreateInvitation,
	CreateJwtTemplate,
	CreateDemoInstance,
	CreateOrganization,
	CreatePhoneNumber,
	CreateRedirectUrl,
	CreateSignInToken,
	CreateUser,
	CreateSvixApp,
	GenerateSvixAuthUrl,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkDeleteEndpoint {
	DeleteSvixApp,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkPatchEndpoint {
	UpdateInstanceAuthConfig,
	UpdateInstance,
	UpdateInstanceOrganizationSettings,
	UpdateInstanceRestrictions,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClerkPutEndpoint {
	UpdateProductionInstanceDomain,
}

impl ClerkGetEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkGetEndpoint::ListAllowlistIdentifiers => "/allowlist_identifiers",
			ClerkGetEndpoint::ListBlocklistIdentifiers => "/blocklist_identifiers",
			ClerkGetEndpoint::GetClientList => "/clients",
			ClerkGetEndpoint::ListInvitations => "/invitations",
			ClerkGetEndpoint::ListJwtTemplates => "/jwt_templates",
			ClerkGetEndpoint::GetPublicInterstitial => "/public/interstitial",
			ClerkGetEndpoint::ListOrganizations => "/organizations",
			ClerkGetEndpoint::ListRedirectUrls => "/redirect_urls",
			ClerkGetEndpoint::GetSessionList => "/sessions",
			ClerkGetEndpoint::GetUserList => "/users",
			ClerkGetEndpoint::GetUsersCount => "/users/count",
		}
	}
}

impl ClerkPostEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkPostEndpoint::CreateActorToken => "/actor_tokens",
			ClerkPostEndpoint::CreateAllowlistIdentifier => "/allowlist_identifiers",
			ClerkPostEndpoint::CreateBlocklistIdentifier => "/blocklist_identifiers",
			ClerkPostEndpoint::VerifyClient => "/clients/verify",
			ClerkPostEndpoint::CreateEmailAddress => "/email_addresses",
			ClerkPostEndpoint::CreateEmail => "/emails",
			ClerkPostEndpoint::CreateInvitation => "/invitations",
			ClerkPostEndpoint::CreateJwtTemplate => "/jwt_templates",
			ClerkPostEndpoint::CreateDemoInstance => "/public/demo_instance",
			ClerkPostEndpoint::CreateOrganization => "/organizations",
			ClerkPostEndpoint::CreatePhoneNumber => "/phone_numbers",
			ClerkPostEndpoint::CreateRedirectUrl => "/redirect_urls",
			ClerkPostEndpoint::CreateSignInToken => "/sign_in_tokens",
			ClerkPostEndpoint::CreateUser => "/users",
			ClerkPostEndpoint::CreateSvixApp => "/webhooks/svix",
			ClerkPostEndpoint::GenerateSvixAuthUrl => "/webhooks/svix_url",
		}
	}
}

impl ClerkDeleteEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkDeleteEndpoint::DeleteSvixApp => "/webhooks/svix",
		}
	}
}

impl ClerkPutEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkPutEndpoint::UpdateProductionInstanceDomain => "/beta_features/domain",
		}
	}
}

impl ClerkPatchEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkPatchEndpoint::UpdateInstanceAuthConfig => "/beta_features/instance_settings",
			ClerkPatchEndpoint::UpdateInstance => "/instance",
			ClerkPatchEndpoint::UpdateInstanceOrganizationSettings => "/instance/organization_settings",
			ClerkPatchEndpoint::UpdateInstanceRestrictions => "/instance/restrictions",
		}
	}
}

impl ClerkDynamicGetEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkDynamicGetEndpoint::GetOrganization => "/organizations/{organization_id}",
			ClerkDynamicGetEndpoint::GetPhoneNumber => "/phone_numbers/{phone_number_id}",
			ClerkDynamicGetEndpoint::GetRedirectUrl => "/redirect_urls/{id}",
			ClerkDynamicGetEndpoint::GetSession => "/sessions/{session_id}",
			ClerkDynamicGetEndpoint::GetUser => "/users/{user_id}",
			ClerkDynamicGetEndpoint::GetClient => "/clients/{client_id}",
			ClerkDynamicGetEndpoint::GetJwks => "/jwks",
			ClerkDynamicGetEndpoint::ListOrganizationMemberships => "/organizations/{organization_id}/memberships",
			ClerkDynamicGetEndpoint::ListPendingOrganizationInvitations => "/organizations/{organization_id}/invitations/pending",
			ClerkDynamicGetEndpoint::GetClientLastActiveSession => "/clients/{client_id}/last_active_session",
			ClerkDynamicGetEndpoint::GetEmailAddress => "/email_addresses/{email_address_id}",
			ClerkDynamicGetEndpoint::GetTemplate => "/templates/{template_type}/{slug}",
			ClerkDynamicGetEndpoint::GetTemplateList => "/templates/{template_type}",
			ClerkDynamicGetEndpoint::GetJwtTemplate => "/jwt_templates/{jwt_template_id}",
			ClerkDynamicGetEndpoint::GetOAuthAccessToken => "/users/{user_id}/oauth_access_tokens/{provider}",
			ClerkDynamicGetEndpoint::UsersGetOrganizationMemberships => "/users/{user_id}/organization_memberships",
		}
	}
}

impl ClerkDynamicPostEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkDynamicPostEndpoint::CreateOrganizationMembership => "/organizations/{organization_id}/memberships",
			ClerkDynamicPostEndpoint::CreateOrganizationInvitation => "/organizations/{organization_id}/invitations",
			ClerkDynamicPostEndpoint::RevokeActorToken => "/actor_tokens/{actor_token_id}/revoke",
			ClerkDynamicPostEndpoint::DeleteBlocklistIdentifier => "/blocklist_identifiers/{identifier_id}",
			ClerkDynamicPostEndpoint::PreviewTemplate => "/templates/{template_type}/{slug}/preview",
			ClerkDynamicPostEndpoint::RevertTemplate => "/templates/{template_type}/{slug}/revert",
			ClerkDynamicPostEndpoint::RevokeInvitation => "/invitations/{invitation_id}/revoke",
			ClerkDynamicPostEndpoint::RevokeOrganizationInvitation => "/organizations/{organization_id}/invitations/{invitation_id}/revoke",
			ClerkDynamicPostEndpoint::CreateSessionTokenFromTemplate => "/sessions/{session_id}/tokens/{template_name}",
			ClerkDynamicPostEndpoint::RevokeSession => "/sessions/{session_id}/revoke",
			ClerkDynamicPostEndpoint::VerifySession => "/sessions/{session_id}/verify",
			ClerkDynamicPostEndpoint::RevokeSignInToken => "/sign_in_tokens/{sign_in_token_id}/revoke",
			ClerkDynamicPostEndpoint::BanUser => "/users/{user_id}/ban",
			ClerkDynamicPostEndpoint::UnbanUser => "/users/{user_id}/unban",
			ClerkDynamicPostEndpoint::VerifyPassword => "/users/{user_id}/verify_password",
			ClerkDynamicPostEndpoint::VerifyTotp => "/users/{user_id}/verify_totp",
		}
	}
}

impl ClerkDynamicDeleteEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkDynamicDeleteEndpoint::DeleteOrganization => "/organizations/{organization_id}",
			ClerkDynamicDeleteEndpoint::DeleteOrganizationMembership => "/organizations/{organization_id}/memberships/{user_id}",
			ClerkDynamicDeleteEndpoint::DeletePhoneNumber => "/phone_numbers/{phone_number_id}",
			ClerkDynamicDeleteEndpoint::DeleteRedirectUrl => "/redirect_urls/{id}",
			ClerkDynamicDeleteEndpoint::DeleteUser => "/users/{user_id}",
			ClerkDynamicDeleteEndpoint::DeleteEmailAddress => "/email_addresses/{email_address_id}",
			ClerkDynamicDeleteEndpoint::DeleteJwtTemplate => "/jwt_templates/{jwt_template_id}",
			ClerkDynamicDeleteEndpoint::DeleteAllowlistIdentifier => "/allowlist_identifiers/{identifier_id}",
			ClerkDynamicDeleteEndpoint::DisableMfa => "/users/{user_id}/disable_mfa",
		}
	}
}

impl ClerkDynamicPutEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkDynamicPutEndpoint::UploadOrganizationLogo => "/organizations/{organization_id}/logo",
			ClerkDynamicPutEndpoint::UpsertTemplate => "/templates/{template_type}/{slug}",
		}
	}
}

impl ClerkDynamicPatchEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn as_str(&self) -> &str {
		match self {
			ClerkDynamicPatchEndpoint::UpdateOrganization => "/organizations/{organization_id}",
			ClerkDynamicPatchEndpoint::UpdateOrganizationMembership => "/organizations/{organization_id}/memberships/{user_id}",
			ClerkDynamicPatchEndpoint::UpdatePhoneNumber => "/phone_numbers/{phone_number_id}",
			ClerkDynamicPatchEndpoint::UpdateUser => "/users/{user_id}",
			ClerkDynamicPatchEndpoint::UpdateEmailAddress => "/email_addresses/{email_address_id}",
			ClerkDynamicPatchEndpoint::UpdateJwtTemplate => "/jwt_templates/{jwt_template_id}",
			ClerkDynamicPatchEndpoint::MergeOrganizationMetadata => "/organizations/{organization_id}/metadata",
			ClerkDynamicPatchEndpoint::UpdateOrganizationMembershipMetadata => "/organizations/{organization_id}/memberships/{user_id}/metadata",
			ClerkDynamicPatchEndpoint::UpdateSignUp => "/sign_ups/{id}",
			ClerkDynamicPatchEndpoint::UpdateUserMetadata => "/users/{user_id}/metadata",
		}
	}
}

impl fmt::Display for ClerkGetEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkPostEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkDeleteEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkPutEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkPatchEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkDynamicGetEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkDynamicPostEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkDynamicDeleteEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkDynamicPutEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl fmt::Display for ClerkDynamicPatchEndpoint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

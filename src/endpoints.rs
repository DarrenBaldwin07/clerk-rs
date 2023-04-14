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
	pub fn to_string(&self) -> String {
		match self {
			ClerkGetEndpoint::ListAllowlistIdentifiers => String::from("/allowlist_identifiers"),
			ClerkGetEndpoint::ListBlocklistIdentifiers => String::from("/blocklist_identifiers"),
			ClerkGetEndpoint::GetClientList => String::from("/clients"),
			ClerkGetEndpoint::ListInvitations => String::from("/invitations"),
			ClerkGetEndpoint::ListJwtTemplates => String::from("/jwt_templates"),
			ClerkGetEndpoint::GetPublicInterstitial => String::from("/public/interstitial"),
			ClerkGetEndpoint::ListOrganizations => String::from("/organizations"),
			ClerkGetEndpoint::ListRedirectUrls => String::from("/redirect_urls"),
			ClerkGetEndpoint::GetSessionList => String::from("/sessions"),
			ClerkGetEndpoint::GetUserList => String::from("/users"),
			ClerkGetEndpoint::GetUsersCount => String::from("/users/count"),
		}
	}
}

impl ClerkPostEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkPostEndpoint::CreateActorToken => String::from("/actor_tokens"),
			ClerkPostEndpoint::CreateAllowlistIdentifier => String::from("/allowlist_identifiers"),
			ClerkPostEndpoint::CreateBlocklistIdentifier => String::from("/blocklist_identifiers"),
			ClerkPostEndpoint::VerifyClient => String::from("/clients/verify"),
			ClerkPostEndpoint::CreateEmailAddress => String::from("/email_addresses"),
			ClerkPostEndpoint::CreateEmail => String::from("/emails"),
			ClerkPostEndpoint::CreateInvitation => String::from("/invitations"),
			ClerkPostEndpoint::CreateJwtTemplate => String::from("/jwt_templates"),
			ClerkPostEndpoint::CreateDemoInstance => String::from("/public/demo_instance"),
			ClerkPostEndpoint::CreateOrganization => String::from("/organizations"),
			ClerkPostEndpoint::CreatePhoneNumber => String::from("/phone_numbers"),
			ClerkPostEndpoint::CreateRedirectUrl => String::from("/redirect_urls"),
			ClerkPostEndpoint::CreateSignInToken => String::from("/sign_in_tokens"),
			ClerkPostEndpoint::CreateUser => String::from("/users"),
			ClerkPostEndpoint::CreateSvixApp => String::from("/webhooks/svix"),
			ClerkPostEndpoint::GenerateSvixAuthUrl => String::from("/webhooks/svix_url"),
		}
	}
}

impl ClerkDeleteEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkDeleteEndpoint::DeleteSvixApp => String::from("/webhooks/svix"),
		}
	}
}

impl ClerkPutEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkPutEndpoint::UpdateProductionInstanceDomain => String::from("/beta_features/domain"),
		}
	}
}

impl ClerkPatchEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkPatchEndpoint::UpdateInstanceAuthConfig => String::from("/beta_features/instance_settings"),
			ClerkPatchEndpoint::UpdateInstance => String::from("/instance"),
			ClerkPatchEndpoint::UpdateInstanceOrganizationSettings => String::from("/instance/organization_settings"),
			ClerkPatchEndpoint::UpdateInstanceRestrictions => String::from("/instance/restrictions"),
		}
	}
}

impl ClerkDynamicGetEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkDynamicGetEndpoint::GetOrganization => String::from("/organizations/{organization_id}"),
			ClerkDynamicGetEndpoint::GetPhoneNumber => String::from("/phone_numbers/{phone_number_id}"),
			ClerkDynamicGetEndpoint::GetRedirectUrl => String::from("/redirect_urls/{id}"),
			ClerkDynamicGetEndpoint::GetSession => String::from("/sessions/{session_id}"),
			ClerkDynamicGetEndpoint::GetUser => String::from("/users/{user_id}"),
			ClerkDynamicGetEndpoint::GetClient => String::from("/clients/{client_id}"),
			ClerkDynamicGetEndpoint::GetJwks => String::from("/jwks"),
			ClerkDynamicGetEndpoint::ListOrganizationMemberships => String::from("/organizations/{organization_id}/memberships"),
			ClerkDynamicGetEndpoint::ListPendingOrganizationInvitations => String::from("/organizations/{organization_id}/invitations/pending"),
			ClerkDynamicGetEndpoint::GetClientLastActiveSession => String::from("/clients/{client_id}/last_active_session"),
			ClerkDynamicGetEndpoint::GetEmailAddress => String::from("/email_addresses/{email_address_id}"),
			ClerkDynamicGetEndpoint::GetTemplate => String::from("/templates/{template_type}/{slug}"),
			ClerkDynamicGetEndpoint::GetTemplateList => String::from("/templates/{template_type}"),
			ClerkDynamicGetEndpoint::GetJwtTemplate => String::from("/jwt_templates/{jwt_template_id}"),
			ClerkDynamicGetEndpoint::GetOAuthAccessToken => String::from("/users/{user_id}/oauth_access_tokens/{provider}"),
			ClerkDynamicGetEndpoint::UsersGetOrganizationMemberships => String::from("/users/{user_id}/organization_memberships"),
		}
	}
}

impl ClerkDynamicPostEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkDynamicPostEndpoint::CreateOrganizationMembership => String::from("/organizations/{organization_id}/memberships"),
			ClerkDynamicPostEndpoint::CreateOrganizationInvitation => String::from("/organizations/{organization_id}/invitations"),
			ClerkDynamicPostEndpoint::RevokeActorToken => String::from("/actor_tokens/{actor_token_id}/revoke"),
			ClerkDynamicPostEndpoint::DeleteBlocklistIdentifier => String::from("/blocklist_identifiers/{identifier_id}"),
			ClerkDynamicPostEndpoint::PreviewTemplate => String::from("/templates/{template_type}/{slug}/preview"),
			ClerkDynamicPostEndpoint::RevertTemplate => String::from("/templates/{template_type}/{slug}/revert"),
			ClerkDynamicPostEndpoint::RevokeInvitation => String::from("/invitations/{invitation_id}/revoke"),
			ClerkDynamicPostEndpoint::RevokeOrganizationInvitation => {
				String::from("/organizations/{organization_id}/invitations/{invitation_id}/revoke")
			}
			ClerkDynamicPostEndpoint::CreateSessionTokenFromTemplate => String::from("/sessions/{session_id}/tokens/{template_name}"),
			ClerkDynamicPostEndpoint::RevokeSession => String::from("/sessions/{session_id}/revoke"),
			ClerkDynamicPostEndpoint::VerifySession => String::from("/sessions/{session_id}/verify"),
			ClerkDynamicPostEndpoint::RevokeSignInToken => String::from("/sign_in_tokens/{sign_in_token_id}/revoke"),
			ClerkDynamicPostEndpoint::BanUser => String::from("/users/{user_id}/ban"),
			ClerkDynamicPostEndpoint::UnbanUser => String::from("/users/{user_id}/unban"),
			ClerkDynamicPostEndpoint::VerifyPassword => String::from("/users/{user_id}/verify_password"),
			ClerkDynamicPostEndpoint::VerifyTotp => String::from("/users/{user_id}/verify_totp"),
		}
	}
}

impl ClerkDynamicDeleteEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkDynamicDeleteEndpoint::DeleteOrganization => String::from("/organizations/{organization_id}"),
			ClerkDynamicDeleteEndpoint::DeleteOrganizationMembership => String::from("/organizations/{organization_id}/memberships/{user_id}"),
			ClerkDynamicDeleteEndpoint::DeletePhoneNumber => String::from("/phone_numbers/{phone_number_id}"),
			ClerkDynamicDeleteEndpoint::DeleteRedirectUrl => String::from("/redirect_urls/{id}"),
			ClerkDynamicDeleteEndpoint::DeleteUser => String::from("/users/{user_id}"),
			ClerkDynamicDeleteEndpoint::DeleteEmailAddress => String::from("/email_addresses/{email_address_id}"),
			ClerkDynamicDeleteEndpoint::DeleteJwtTemplate => String::from("/jwt_templates/{jwt_template_id}"),
			ClerkDynamicDeleteEndpoint::DeleteAllowlistIdentifier => String::from("/allowlist_identifiers/{identifier_id}"),
			ClerkDynamicDeleteEndpoint::DisableMfa => String::from("/users/{user_id}/disable_mfa"),
		}
	}
}

impl ClerkDynamicPutEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkDynamicPutEndpoint::UploadOrganizationLogo => String::from("/organizations/{organization_id}/logo"),
			ClerkDynamicPutEndpoint::UpsertTemplate => String::from("/templates/{template_type}/{slug}"),
		}
	}
}

impl ClerkDynamicPatchEndpoint {
	/// Convert a clerk endpoint enum to a string value
	pub fn to_string(&self) -> String {
		match self {
			ClerkDynamicPatchEndpoint::UpdateOrganization => String::from("/organizations/{organization_id}"),
			ClerkDynamicPatchEndpoint::UpdateOrganizationMembership => String::from("/organizations/{organization_id}/memberships/{user_id}"),
			ClerkDynamicPatchEndpoint::UpdatePhoneNumber => String::from("/phone_numbers/{phone_number_id}"),
			ClerkDynamicPatchEndpoint::UpdateUser => String::from("/users/{user_id}"),
			ClerkDynamicPatchEndpoint::UpdateEmailAddress => String::from("/email_addresses/{email_address_id}"),
			ClerkDynamicPatchEndpoint::UpdateJwtTemplate => String::from("/jwt_templates/{jwt_template_id}"),
			ClerkDynamicPatchEndpoint::MergeOrganizationMetadata => String::from("/organizations/{organization_id}/metadata"),
			ClerkDynamicPatchEndpoint::UpdateOrganizationMembershipMetadata => {
				String::from("/organizations/{organization_id}/memberships/{user_id}/metadata")
			}
			ClerkDynamicPatchEndpoint::UpdateSignUp => String::from("/sign_ups/{id}"),
			ClerkDynamicPatchEndpoint::UpdateUserMetadata => String::from("/users/{user_id}/metadata"),
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

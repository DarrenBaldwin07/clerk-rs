use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkDynamicGetEndpoint {
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
    UsersGetOrganizationMemberships
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkDynamicPostEndpoint {
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
    VerifyTotp
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkDynamicDeleteEndpoint {
    DeleteAllowlistIdentifier,
    DeleteEmailAddress,
    DeleteJwtTemplate,
    DeleteOrganizationMembership,
    DeleteOrganization,
    DeletePhoneNumber,
    DeleteRedirectUrl,
    DeleteUser,
    DisableMfa
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkDynamicPutEndpoint {
    UpsertTemplate,
    UploadOrganizationLogo
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkDynamicPatchEndpoint {
    UpdateEmailAddress,
    UpdateJwtTemplate,
    UpdateOrganizationMembershipMetadata,
    MergOrganizationMetadata,
    UpdateOrganizationMembership,
    UpdateOrganization,
    UpdatePhoneNumber,
    UpdateSignUp,
    UpdateUser,
    UpdateUserMetadata
}


#[derive(Debug)]
#[allow(dead_code)]
enum ClerkGetEndpoint {
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
    GetUsersCount
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkPostEndpoint {
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
    GenerateSvixAuthUrl
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkDeleteEndpoint {
    DeleteSvixApp
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkPatchEndpoint {
    UpdateInstanceAuthConfig,
    UpdateInstance,
    UpdateInstanceOrganizationSettings,
    UpdateInstanceRestrictions,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ClerkPutEndpoint {
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
            ClerkGetEndpoint::GetPublicInterstitial => String::from("/public_interstitial"),
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

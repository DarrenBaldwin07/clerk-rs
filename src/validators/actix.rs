use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation, decode_header};
use actix_web::{HttpResponse, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
Error, http::header::HeaderValue,};
use std::{future::{ready, Ready}, rc::Rc};
use crate::{clerk::Clerk, apis::jwks_api::{JwksModel, Jwks}, ClerkConfiguration};
use futures_util::future::LocalBoxFuture;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkJwt {
    pub azp: String,
    pub exp: i32,
    pub iat: i32,
    pub iss: String,
    pub nbf: i32,
    pub sid: String,
    pub sub: String
}


/// Extract token kid from a jwt passed as header to an actix-web endpoint
pub fn token_kid(jwt: String) -> String {
    let header = decode_header(&jwt).expect("Error: could not parse token kid!");
    header.kid.expect("Error: Token doesn't have a `kid` header field!")
}

/// Validate a jwt using a jwks set
pub fn validate_jwt(token: &str, jwks: JwksModel) -> bool {
    let parsed_jwt = token.replace("Bearer ", "");
    let kid = token_kid(parsed_jwt.to_owned());

    let jwk = jwks.keys.iter().find(|k| k.kid == kid);

    // Check to see if we found a valid jwk key with the token kid
    if let Some(j) = jwk {
        match j.alg.as_str() {
            // Currently, clerk only supports Rs256 by default (could be wrong here -- might need to look into this)
           "RS256" => {
                let decoding_key = DecodingKey::from_rsa_components(&j.n, &j.e).unwrap();
                let mut validation = Validation::new(Algorithm::RS256);
                validation.validate_exp = true;
                let decoded_token =
                    decode::<ClerkJwt>(&parsed_jwt, &decoding_key, &validation);

                return match decoded_token {
                    Ok(_) => true,
                    _ => false
                };
            }
            _ => unreachable!("this should be a RSA"),
        }
    // In the event that a matching jwk was not found we want to output a false result (showing that the jwt was invalid)
    } else {
        return false;
    }
}

/// Authorize a actix-web route given a `clerk_client` and a valid service request to an actix-web endpoint
pub async fn clerk_authorize(req: &ServiceRequest, clerk_client: &Clerk) -> Result<bool, HttpResponse> {
    // Get our jwks from Clerk.dev
    let jwks = match Jwks::get_jwks(clerk_client).await {
        Ok(val) => val,
        Err(_) => return Err(HttpResponse::InternalServerError().json("Error: Could not fetch JWKS!"))
    };
    // Parse the request headers
    let access_token: &str = match req.headers().get("Authorization").unwrap().to_str() {
        Ok(val) => val,
        Err(_) => return Err(HttpResponse::InternalServerError().json("Error: No Authorization header found on the request payload!"))
    };

    // Finally, check if the jwt is valid...
    let is_valid_jwt = validate_jwt(access_token, jwks);

    Ok(is_valid_jwt)
}


pub fn parse_cookies(req: &ServiceRequest) -> Option<&HeaderValue> {
    req.headers().get("cookie")
}


/// Actix-web middleware for protecting a http endpoint with Cerk.dev
pub struct ClerkMiddleware;


impl<S: 'static, B> Transform<S, ServiceRequest> for ClerkMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ClerkMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ClerkMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct ClerkMiddlewareService<S> {
    service: Rc<S>,
}

impl<S: 'static, B> Service<ServiceRequest> for ClerkMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        // Create a new clerk configuration so that we can make authed requests
        let config = ClerkConfiguration::new(None, None, Some("sk_test_iMGT8MozGDT47hkZd2FyEeoSKtE6MTy9f2tpVjkRUy".to_string()), None);
        // Initialize our Clerk client with the newly created configuration
        let client = Clerk::new(config);


        let svc = self.service.clone();
        Box::pin(async move {
            let is_authed = clerk_authorize(&req, &client).await;

			let res = svc.call(req).await?;
			Ok(res)
		})
    }
}



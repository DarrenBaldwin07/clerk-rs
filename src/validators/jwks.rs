use crate::{
	apis::jwks_api::{Jwks, JwksKey},
	clerk::Clerk,
	validators::authorizer::ClerkError,
};
use arc_swap::{ArcSwap, Guard};
use async_trait::async_trait;
use std::{
	collections::HashMap,
	sync::Arc,
	time::{Duration, SystemTime},
};

/// Trait that implements a provider for the JWKS keys, to be used when validating a JWT.
///
/// This crate provides two basic implementations of this trait, [`MemoryCacheJwksProvider`] and [`SimpleJwksProvider`].
/// By implementing `get_key` for your own struct you can customize how the validator fetches keys.
#[async_trait]
pub trait JwksProvider {
	type Error: Into<ClerkError>;

	async fn get_key(&self, kid: &str) -> Result<JwksKey, Self::Error>;
}

/// Error type used by [`MemoryCacheJwksProvider`] and [`SimpleJwksProvider`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JwksProviderError {
	UnknownKey,
	JwksApi,
}

impl From<JwksProviderError> for ClerkError {
	fn from(e: JwksProviderError) -> Self {
		match e {
			JwksProviderError::UnknownKey => ClerkError::Unauthorized("Error: Invalid JWT!".into()),
			JwksProviderError::JwksApi => ClerkError::InternalServerError(String::from("Error: Could not fetch JWKS!")),
		}
	}
}

/// A [`JwksProvider`] implementation that doesn't do any caching.
///
/// The JWKS is fetched from the Clerk API on every request.
pub struct JwksProviderNoCache<'a> {
	clerk_client: &'a Clerk,
}

impl<'a> JwksProviderNoCache<'a> {
	pub fn new(clerk_client: &'a Clerk) -> Self {
		Self { clerk_client }
	}
}

#[async_trait]
impl JwksProvider for JwksProviderNoCache<'_> {
	type Error = JwksProviderError;

	async fn get_key(&self, kid: &str) -> Result<JwksKey, JwksProviderError> {
		let jwks = Jwks::get_jwks(self.clerk_client).await.map_err(|_| JwksProviderError::JwksApi)?;

		jwks.keys.into_iter().find(|k| k.kid == kid).ok_or(JwksProviderError::UnknownKey)
	}
}

/// Configures how [`MemoryCacheJwksProvider`] handles requests for non-cached keys.
pub enum RefreshOnUnknown {
	/// Never attempt to refresh the JWKS when an unknown `kid` is requested.
	Never,
	/// Attempt to refresh the JWKS when an unknown `kid` is requested
	/// if the cached value is older than the given duration.
	Ratelimit(Duration),
	/// Always attempt to refresh the JWKS when an unknown `kid` is requested.
	Always,
}

/// Options for [`MemoryCacheJwksProvider`].
pub struct MemoryCacheJwksProviderOptions {
	/// How long to cache the JWKS for.
	/// If this is None, the cached JWKS will live forever or until it is replaced due to [`RefreshOnUnknown`].
	///
	/// Defaults to 1 hour.
	pub expire_after: Option<Duration>,
	/// Configures the behavior of the provider when a `kid` that isn't in the cache is requested.
	///
	/// Defaults to refreshing when an unknown `kid` is requested, at most every 5 minutes.
	pub refresh_on_unknown: RefreshOnUnknown,
}

impl Default for MemoryCacheJwksProviderOptions {
	fn default() -> Self {
		Self {
			// 1 hour
			expire_after: Some(Duration::from_secs(60 * 60)),
			// 5 minutes
			refresh_on_unknown: RefreshOnUnknown::Ratelimit(Duration::from_secs(60 * 5)),
		}
	}
}

// Internal state for MemoryCacheJwksProvider.
// The provider holds this struct in an ArcSwap to allow atomically replacing it.
struct MemoryCacheJwksProviderState {
	keys: HashMap<String, JwksKey>,
	last_updated: SystemTime,
}

impl MemoryCacheJwksProviderState {
	fn is_uninitialized(&self) -> bool {
		self.last_updated == SystemTime::UNIX_EPOCH
	}

	fn is_expired(&self, expire_after: Option<Duration>) -> bool {
		// if expire_after is None, the cache is never expired
		let Some(expire_after) = expire_after else { return false };

		let Ok(elapsed) = self.last_updated.elapsed() else {
			// time went backwards, return expired
			return true;
		};

		elapsed >= expire_after
	}
}

/// A [`JwksProvider`] implementation that caches keys in memory.
///
/// The JWKS is fetched from the Clerk API on the first request or when the cache expires.
/// Cache behavior can be configured with [`MemoryCacheJwksProviderOptions`].
pub struct MemoryCacheJwksProvider<'a> {
	clerk_client: &'a Clerk,
	options: MemoryCacheJwksProviderOptions,
	state: ArcSwap<MemoryCacheJwksProviderState>,
}

impl<'a> MemoryCacheJwksProvider<'a> {
	/// Creates a new [`MemoryCacheJwksProvider`] with the given client and the default options.
	pub fn new(clerk_client: &'a Clerk) -> Self {
		Self::new_with_options(clerk_client, MemoryCacheJwksProviderOptions::default())
	}

	/// Creates a new [`MemoryCacheJwksProvider`] with the given client and options.
	pub fn new_with_options(clerk_client: &'a Clerk, options: MemoryCacheJwksProviderOptions) -> Self {
		let initial_state = MemoryCacheJwksProviderState {
			keys: HashMap::new(),
			last_updated: SystemTime::UNIX_EPOCH, // mark uninitialized
		};

		Self {
			clerk_client,
			options,
			state: ArcSwap::new(Arc::new(initial_state)),
		}
	}

	async fn refresh(&self) -> Result<Arc<MemoryCacheJwksProviderState>, JwksProviderError> {
		// fetch jwks from clerk api
		let jwks_model = Jwks::get_jwks(self.clerk_client).await.map_err(|_| JwksProviderError::JwksApi)?;

		// construct new state
		let keys = jwks_model.keys.into_iter().map(|k| (k.kid.clone(), k)).collect();
		let state = MemoryCacheJwksProviderState {
			keys,
			last_updated: SystemTime::now(),
		};

		Ok(Arc::new(state))
	}
}

#[async_trait]
impl JwksProvider for MemoryCacheJwksProvider<'_> {
	type Error = JwksProviderError;

	async fn get_key(&self, kid: &str) -> Result<JwksKey, Self::Error> {
		// get the current cache state
		let state = self.state.load();

		// if the state is uninitialized or expired, refresh it
		let mut refreshed = false;
		let state = if state.is_uninitialized() || state.is_expired(self.options.expire_after) {
			// attempt to refresh the jwks and store the new cache state
			let new_state = self.refresh().await?;
			self.state.swap(new_state.clone());
			refreshed = true;
			Guard::from_inner(new_state)
		} else {
			state
		};

		let maybe_key = state.keys.get(kid).cloned();

		if let Some(key) = maybe_key {
			// key found in cache
			Ok(key)
		} else {
			// key not in cache, do stuff depending on refresh_on_unknown

			// if we just refreshed, don't refresh again
			if refreshed {
				return Err(JwksProviderError::UnknownKey);
			}

			// if refresh_on_unknown is Never, just error
			if let RefreshOnUnknown::Never = self.options.refresh_on_unknown {
				return Err(JwksProviderError::UnknownKey);
			}

			// if refresh_on_unknown is Ratelimit and the cache is too new, error
			if let RefreshOnUnknown::Ratelimit(min_age) = self.options.refresh_on_unknown {
				if !state.is_expired(Some(min_age)) {
					return Err(JwksProviderError::UnknownKey);
				}
			}

			// ratelimit is not exceeded or refresh_on_unkown is Always, so refresh and store
			let new_state = self.refresh().await?;
			self.state.swap(new_state.clone());

			// check if the new state has the key
			new_state.keys.get(kid).cloned().ok_or(JwksProviderError::UnknownKey)
		}
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::{apis::jwks_api::JwksKey, ClerkConfiguration};
	use std::collections::HashMap;

	pub struct StaticJwksProvider {
		keys: HashMap<String, JwksKey>,
	}

	impl StaticJwksProvider {
		pub fn from_key(key: JwksKey) -> Self {
			let mut keys = HashMap::new();
			keys.insert(key.kid.clone(), key);

			Self { keys }
		}
	}

	#[async_trait]
	impl JwksProvider for StaticJwksProvider {
		type Error = JwksProviderError;

		async fn get_key(&self, kid: &str) -> Result<JwksKey, Self::Error> {
			self.keys.get(kid).cloned().ok_or(JwksProviderError::UnknownKey)
		}
	}

	const MOCK_JWKS_BODY: &str = r#"{
		"keys": [{
			"use": "sig",
			"kty": "RSA",
			"kid": "bc63c2e9-5d1c-4e32-9b62-178f60409abd",
			"alg": "RS256",
			"n": "tgY-zUiCj6p4gDLZos28PJXyBimvDCnvlCxpG8jktCJdbw1VrsAR1tqmz3XrKnpXgKuWaBnoAh9SpslSN-lQhHT_KVHgVUQShMETybKrNx9DoeRChwen26n35BOLCtZE7yUamUGrQpcL4DsL8ZmWcllOLCWjHRenuXJohoQO7jKN9tao2mUpsRor-2O1xKZ_YesCDkDHw7ood4lfNKvDONB8gYIENlOJgAbAKPxTdmnkEraUgZVGeaS7FeB59A_ibj9VnXyqpHmhabSf5xskuA9EJiQn6c3781uGqcF2CS0E4I576oJGsKeKo5AgF2duuDnPd67bVRNvmLH5kDF_Ow",
			"e": "AQAB"
		}]
	}"#;
	const MOCK_KID: &str = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

	#[tokio::test]
	async fn test_simple_jwks_provider_success() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(1).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = JwksProviderNoCache::new(&clerk);

		let res = jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		assert_eq!(res.kid, MOCK_KID);

		// api should have been called once
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_simple_jwks_provider_repeat() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(3).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = JwksProviderNoCache::new(&clerk);

		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");

		// api should have been called exactly 3 times
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_simple_jwks_provider_unknown_key() {
		let mut server = mockito::Server::new_async().await;
		server.mock("GET", "/v1/jwks").with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = JwksProviderNoCache::new(&clerk);

		// try to get a key that doesn't exist
		let res = jwks.get_key("unknown key").await.expect_err("should fail");
		assert_eq!(res, JwksProviderError::UnknownKey)
	}

	#[tokio::test]
	async fn test_memory_cache_jwks_provider_success() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(1).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = MemoryCacheJwksProvider::new(&clerk);

		let res = jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		assert_eq!(res.kid, MOCK_KID);

		// api should have been called once
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_memory_cache_jwks_provider_caching() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(1).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = MemoryCacheJwksProvider::new(&clerk);

		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");

		// api should have been called only once
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_memory_cache_jwks_provider_unknown_never() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(1).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = MemoryCacheJwksProvider::new_with_options(
			&clerk,
			MemoryCacheJwksProviderOptions {
				refresh_on_unknown: RefreshOnUnknown::Never,
				..Default::default()
			},
		);

		// first request to initialize cache
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");

		// second request with unknown key should NOT refresh
		jwks.get_key("unknown key").await.expect_err("should fail");

		// since refresh_on_unknown is Never, api should have been called only once
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_memory_cache_jwks_provider_unknown_refresh() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(3).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = MemoryCacheJwksProvider::new_with_options(
			&clerk,
			MemoryCacheJwksProviderOptions {
				refresh_on_unknown: RefreshOnUnknown::Always,
				..Default::default()
			},
		);

		// first request to initialize cache
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");

		// following requests with unknown keys refresh
		jwks.get_key("unknown key 1").await.expect_err("should fail");
		jwks.get_key("unknown key 2").await.expect_err("should fail");

		// since refresh_on_unknown is Always, api should have been called three times
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_memory_cache_jwks_provider_unknown_ratelimit() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(3).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = MemoryCacheJwksProvider::new_with_options(
			&clerk,
			MemoryCacheJwksProviderOptions {
				refresh_on_unknown: RefreshOnUnknown::Ratelimit(Duration::from_secs(1)),
				..Default::default()
			},
		);

		// first request to initialize cache
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");

		// wait for cache to be older than the 1s ratelimit
		tokio::time::sleep(Duration::from_secs(2)).await;

		// the first unknown key should cause a refresh, but following ones should not
		jwks.get_key("unknown key 1").await.expect_err("should fail");
		jwks.get_key("unknown key 2").await.expect_err("should fail");
		jwks.get_key("unknown key 3").await.expect_err("should fail");

		// wait for ratelimit again
		tokio::time::sleep(Duration::from_secs(2)).await;

		// again only the first should refresh
		jwks.get_key("unknown key 4").await.expect_err("should fail");
		jwks.get_key("unknown key 5").await.expect_err("should fail");
		jwks.get_key("unknown key 6").await.expect_err("should fail");

		// api should have been called exactly 3 times
		mock.assert_async().await;
	}

	#[tokio::test]
	async fn test_memory_cache_jwks_provider_expires() {
		let mut server = mockito::Server::new_async().await;
		let mock = server.mock("GET", "/v1/jwks").expect(2).with_body(MOCK_JWKS_BODY).create_async().await;

		let config = ClerkConfiguration {
			base_path: format!("{}/v1", server.url()),
			..Default::default()
		};
		let clerk = Clerk::new(config);

		let jwks = MemoryCacheJwksProvider::new_with_options(
			&clerk,
			MemoryCacheJwksProviderOptions {
				expire_after: Some(Duration::from_secs(1)),
				..Default::default()
			},
		);

		// make some requests
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");

		// wait for cache to expire
		tokio::time::sleep(Duration::from_secs(2)).await;

		// make more requests
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");
		jwks.get_key(MOCK_KID).await.expect("should retrieve key");

		// api should have been called only 2 times
		mock.assert_async().await;
	}
}

/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequest {
	/// The ID of the user as used in your external systems or your previous authentication solution. Must be unique across your instance.
	#[serde(
		rename = "external_id",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub external_id: Option<Option<String>>,
	/// The first name to assign to the user
	#[serde(
		rename = "first_name",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub first_name: Option<Option<String>>,
	/// The last name to assign to the user
	#[serde(
		rename = "last_name",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub last_name: Option<Option<String>>,
	/// Email addresses to add to the user. Must be unique across your instance. The first email address will be set as the user's primary email address.
	#[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
	pub email_address: Option<Vec<String>>,
	/// Phone numbers to add to the user. Must be unique across your instance. The first phone number will be set as the user's primary phone number.
	#[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
	pub phone_number: Option<Vec<String>>,
	/// Web3 wallets to add to the user. Must be unique across your instance. The first wallet will be set as the user's primary wallet.
	#[serde(rename = "web3_wallet", skip_serializing_if = "Option::is_none")]
	pub web3_wallet: Option<Vec<String>>,
	/// The username to give to the user. It must be unique across your instance.
	#[serde(
		rename = "username",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub username: Option<Option<String>>,
	/// The plaintext password to give the user. Must be at least 8 characters long, and can not be in any list of hacked passwords.
	#[serde(
		rename = "password",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub password: Option<Option<String>>,
	/// In case you already have the password digests and not the passwords, you can use them for the newly created user via this property. The digests should be generated with one of the supported algorithms. The hashing algorithm can be specified using the `password_hasher` property.
	#[serde(rename = "password_digest", skip_serializing_if = "Option::is_none")]
	pub password_digest: Option<String>,
	/// The hashing algorithm that was used to generate the password digest. The algorithms we support at the moment are [bcrypt](https://en.wikipedia.org/wiki/Bcrypt), [bcrypt_sha256_django](https://docs.djangoproject.com/en/4.0/topics/auth/passwords/), [md5](https://en.wikipedia.org/wiki/MD5), pbkdf2_sha256, [pbkdf2_sha256_django](https://docs.djangoproject.com/en/4.0/topics/auth/passwords/), [phpass](https://www.openwall.com/phpass/), [scrypt_firebase](https://firebaseopensource.com/projects/firebase/scrypt/), [sha256](https://en.wikipedia.org/wiki/SHA-2) and the [argon2](https://argon2.online/) variants argon2i and argon2id.  If you need support for any particular hashing algorithm, [please let us know](https://clerk.com/support).  Note: for password hashers considered insecure (at this moment MD5 and SHA256), the corresponding user password hashes will be transparently migrated to Bcrypt (a secure hasher) upon the user's first successful password sign in. Insecure schemes are marked with `(insecure)` in the list below.  Each of the supported hashers expects the incoming digest to be in a particular format. Specifically:  **bcrypt:** The digest should be of the following form:  `$<algorithm version>$<cost>$<salt & hash>`  **bcrypt_sha256_django:** This is the Django-specific variant of Bcrypt, using SHA256 hashing function. The format should be as follows (as exported from Django):  `bcrypt_sha256$$<algorithm version>$<cost>$<salt & hash>`  **md5** (insecure): The digest should follow the regular form e.g.:  `5f4dcc3b5aa765d61d8327deb882cf99`  **pbkdf2_sha256:** This is the PBKDF2 algorithm using the SHA256 hashing function. The format should be as follows:  `pbkdf2_sha256$<iterations>$<salt>$<hash>`  Note: Both the salt and the hash are expected to be base64-encoded.  **pbkdf2_sha256_django:** This is the Django-specific variant of PBKDF2 and the digest should have the following format (as exported from Django):  `pbkdf2_sha256$<iterations>$<salt>$<hash>`  Note: The salt is expected to be un-encoded, the hash is expected base64-encoded.  **pbkdf2_sha1:** This is similar to pkbdf2_sha256_django, but with two differences: 1. uses sha1 instead of sha256 2. accepts the hash as a hex-encoded string  The format is the following:  `pbkdf2_sha1$<iterations>$<salt>$<hash-as-hex-string>`  **phpass:** Portable public domain password hashing framework for use in PHP applications. Digests hashed with phpass have the following sections:  The format is the following:  `$P$<rounds><salt><encoded-checksum>`  - $P$ is the prefix used to identify phpass hashes. - rounds is a single character encoding a 6-bit integer representing the number of rounds used. - salt is eight characters drawn from [./0-9A-Za-z], providing a 48-bit salt. - checksum is 22 characters drawn from the same set, encoding the 128-bit checksum with MD5.  **scrypt_firebase:** The Firebase-specific variant of scrypt. The value is expected to have 6 segments separated by the $ character and include the following information:  _hash:_ The actual Base64 hash. This can be retrieved when exporting the user from Firebase. _salt:_ The salt used to generate the above hash. Again, this is given when exporting the user. _signer key:_ The base64 encoded signer key. _salt separator:_ The base64 encoded salt separator. _rounds:_ The number of rounds the algorithm needs to run. _memory cost:_ The cost of the algorithm run  The first 2 (hash and salt) are per user and can be retrieved when exporting the user from Firebase. The other 4 values (signer key, salt separator, rounds and memory cost) are project-wide settings and can be retrieved from the project's password hash parameters.  Once you have all these, you can combine it in the following format and send this as the digest in order for Clerk to accept it:  `<hash>$<salt>$<signer key>$<salt separator>$<rounds>$<memory cost>`  **argon2i:** Algorithms in the argon2 family generate digests that encode the following information:  _version (v):_ The argon version, version 19 is assumed _memory (m):_ The memory used by the algorithm (in kibibytes) _iterations (t):_ The number of iterations to perform _parallelism (p):_ The number of threads to use  Parts are demarcated by the `$` character, with the first part identifying the algorithm variant. The middle part is a comma-separated list of the encoding options (memory, iterations, parallelism). The final part is the actual digest.  `$argon2i$v=19$m=4096,t=3,p=1$4t6CL3P7YiHBtwESXawI8Hm20zJj4cs7/4/G3c187e0$m7RQFczcKr5bIR0IIxbpO2P0tyrLjf3eUW3M3QSwnLc`  **argon2id:** See the previous algorithm for an explanation of the formatting.  For the argon2id case, the value of the algorithm in the first part of the digest is `argon2id`:  `$argon2id$v=19$m=64,t=4,p=8$Z2liZXJyaXNo$iGXEpMBTDYQ8G/71tF0qGjxRHEmR3gpGULcE93zUJVU`  **sha256** (insecure): The digest should be a 64-length hex string, e.g.:  `9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08`
	#[serde(rename = "password_hasher", skip_serializing_if = "Option::is_none")]
	pub password_hasher: Option<PasswordHasher>,
	/// When set to `true` all password checks are skipped. 
	/// 
	/// SECURITY WARNING: This is a high-risk operation that bypasses password security checks and should 
	/// ONLY be used for controlled migration scenarios. Using this flag can lead to security vulnerabilities.
	/// It is strongly recommended to:
	///  1. Only use this method when migrating plaintext passwords to Clerk
	///  2. Prompt users to pick stronger passwords after migration
	///  3. Log and audit all instances where this flag is used
	///  4. Never use in production for regular user creation
	#[serde(rename = "skip_password_checks", skip_serializing_if = "Option::is_none")]
	pub skip_password_checks: Option<bool>,
	/// When set to `true`, `password` is not required anymore when creating the user and can be omitted. 
	/// 
	/// SECURITY WARNING: This is a high-risk operation that bypasses password requirements and should
	/// ONLY be used in specific controlled scenarios. Using this flag can lead to security vulnerabilities.
	/// It is strongly recommended to:
	///  1. Only use for migration or specific authorized use cases
	///  2. Log and audit all instances where this flag is used
	///  3. Verify this won't compromise your authentication security
	/// 
	/// Please note that you cannot use this flag if password is the only way for a user to sign into your instance.
	#[serde(rename = "skip_password_requirement", skip_serializing_if = "Option::is_none")]
	pub skip_password_requirement: Option<bool>,
	/// In case TOTP is configured on the instance, you can provide the secret to enable it on the newly created user without the need to reset it. Please note that currently the supported options are: * Period: 30 seconds * Code length: 6 digits * Algorithm: SHA1
	#[serde(rename = "totp_secret", skip_serializing_if = "Option::is_none")]
	pub totp_secret: Option<String>,
	/// If Backup Codes are configured on the instance, you can provide them to enable it on the newly created user without the need to reset them. You must provide the backup codes in plain format or the corresponding bcrypt digest.
	#[serde(rename = "backup_codes", skip_serializing_if = "Option::is_none")]
	pub backup_codes: Option<Vec<String>>,
	/// Metadata saved on the user, that is visible to both your Frontend and Backend APIs
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	/// Metadata saved on the user, that is only visible to your Backend API
	#[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
	pub private_metadata: Option<serde_json::Value>,
	/// Metadata saved on the user, that can be updated from both the Frontend and Backend APIs. Note: Since this data can be modified from the frontend, it is not guaranteed to be safe.
	#[serde(rename = "unsafe_metadata", skip_serializing_if = "Option::is_none")]
	pub unsafe_metadata: Option<serde_json::Value>,
	/// A custom date/time denoting _when_ the user signed up to the application, specified in RFC3339 format (e.g. `2012-10-20T07:15:20.902Z`).
	#[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
	pub created_at: Option<String>,
}

impl CreateUserRequest {
	pub fn new() -> CreateUserRequest {
		CreateUserRequest {
			external_id: None,
			first_name: None,
			last_name: None,
			email_address: None,
			phone_number: None,
			web3_wallet: None,
			username: None,
			password: None,
			password_digest: None,
			password_hasher: None,
			skip_password_checks: None,
			skip_password_requirement: None,
			totp_secret: None,
			backup_codes: None,
			public_metadata: None,
			private_metadata: None,
			unsafe_metadata: None,
			created_at: None,
		}
	}
}

/// The hashing algorithm that was used to generate the password digest. The algorithms we support at the moment are [bcrypt](https://en.wikipedia.org/wiki/Bcrypt), [bcrypt_sha256_django](https://docs.djangoproject.com/en/4.0/topics/auth/passwords/), [md5](https://en.wikipedia.org/wiki/MD5), pbkdf2_sha256, [pbkdf2_sha256_django](https://docs.djangoproject.com/en/4.0/topics/auth/passwords/), [phpass](https://www.openwall.com/phpass/), [scrypt_firebase](https://firebaseopensource.com/projects/firebase/scrypt/), [sha256](https://en.wikipedia.org/wiki/SHA-2) and the [argon2](https://argon2.online/) variants argon2i and argon2id.  If you need support for any particular hashing algorithm, [please let us know](https://clerk.com/support).  Note: for password hashers considered insecure (at this moment MD5 and SHA256), the corresponding user password hashes will be transparently migrated to Bcrypt (a secure hasher) upon the user's first successful password sign in. Insecure schemes are marked with `(insecure)` in the list below.  Each of the supported hashers expects the incoming digest to be in a particular format. Specifically:  **bcrypt:** The digest should be of the following form:  `$<algorithm version>$<cost>$<salt & hash>`  **bcrypt_sha256_django:** This is the Django-specific variant of Bcrypt, using SHA256 hashing function. The format should be as follows (as exported from Django):  `bcrypt_sha256$$<algorithm version>$<cost>$<salt & hash>`  **md5** (insecure): The digest should follow the regular form e.g.:  `5f4dcc3b5aa765d61d8327deb882cf99`  **pbkdf2_sha256:** This is the PBKDF2 algorithm using the SHA256 hashing function. The format should be as follows:  `pbkdf2_sha256$<iterations>$<salt>$<hash>`  Note: Both the salt and the hash are expected to be base64-encoded.  **pbkdf2_sha256_django:** This is the Django-specific variant of PBKDF2 and the digest should have the following format (as exported from Django):  `pbkdf2_sha256$<iterations>$<salt>$<hash>`  Note: The salt is expected to be un-encoded, the hash is expected base64-encoded.  **pbkdf2_sha1:** This is similar to pkbdf2_sha256_django, but with two differences: 1. uses sha1 instead of sha256 2. accepts the hash as a hex-encoded string  The format is the following:  `pbkdf2_sha1$<iterations>$<salt>$<hash-as-hex-string>`  **phpass:** Portable public domain password hashing framework for use in PHP applications. Digests hashed with phpass have the following sections:  The format is the following:  `$P$<rounds><salt><encoded-checksum>`  - $P$ is the prefix used to identify phpass hashes. - rounds is a single character encoding a 6-bit integer representing the number of rounds used. - salt is eight characters drawn from [./0-9A-Za-z], providing a 48-bit salt. - checksum is 22 characters drawn from the same set, encoding the 128-bit checksum with MD5.  **scrypt_firebase:** The Firebase-specific variant of scrypt. The value is expected to have 6 segments separated by the $ character and include the following information:  _hash:_ The actual Base64 hash. This can be retrieved when exporting the user from Firebase. _salt:_ The salt used to generate the above hash. Again, this is given when exporting the user. _signer key:_ The base64 encoded signer key. _salt separator:_ The base64 encoded salt separator. _rounds:_ The number of rounds the algorithm needs to run. _memory cost:_ The cost of the algorithm run  The first 2 (hash and salt) are per user and can be retrieved when exporting the user from Firebase. The other 4 values (signer key, salt separator, rounds and memory cost) are project-wide settings and can be retrieved from the project's password hash parameters.  Once you have all these, you can combine it in the following format and send this as the digest in order for Clerk to accept it:  `<hash>$<salt>$<signer key>$<salt separator>$<rounds>$<memory cost>`  **argon2i:** Algorithms in the argon2 family generate digests that encode the following information:  _version (v):_ The argon version, version 19 is assumed _memory (m):_ The memory used by the algorithm (in kibibytes) _iterations (t):_ The number of iterations to perform _parallelism (p):_ The number of threads to use  Parts are demarcated by the `$` character, with the first part identifying the algorithm variant. The middle part is a comma-separated list of the encoding options (memory, iterations, parallelism). The final part is the actual digest.  `$argon2i$v=19$m=4096,t=3,p=1$4t6CL3P7YiHBtwESXawI8Hm20zJj4cs7/4/G3c187e0$m7RQFczcKr5bIR0IIxbpO2P0tyrLjf3eUW3M3QSwnLc`  **argon2id:** See the previous algorithm for an explanation of the formatting.  For the argon2id case, the value of the algorithm in the first part of the digest is `argon2id`:  `$argon2id$v=19$m=64,t=4,p=8$Z2liZXJyaXNo$iGXEpMBTDYQ8G/71tF0qGjxRHEmR3gpGULcE93zUJVU`  **sha256** (insecure): The digest should be a 64-length hex string, e.g.:  `9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PasswordHasher {
	#[serde(rename = "argon2i")]
	Argon2i,
	#[serde(rename = "argon2id")]
	Argon2id,
	#[serde(rename = "bcrypt")]
	Bcrypt,
	#[serde(rename = "bcrypt_sha256_django")]
	BcryptSha256Django,
	#[serde(rename = "md5")]
	Md5,
	#[serde(rename = "pbkdf2_sha256")]
	Pbkdf2Sha256,
	#[serde(rename = "pbkdf2_sha256_django")]
	Pbkdf2Sha256Django,
	#[serde(rename = "pbkdf2_sha1")]
	Pbkdf2Sha1,
	#[serde(rename = "phpass")]
	Phpass,
	#[serde(rename = "scrypt_firebase")]
	ScryptFirebase,
	#[serde(rename = "sha256")]
	Sha256,
}

impl Default for PasswordHasher {
	fn default() -> PasswordHasher {
		Self::Argon2i
	}
}

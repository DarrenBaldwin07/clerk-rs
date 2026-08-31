# UpdateUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_id** | Option<**String**> | The ID of the user as used in your external systems or your previous authentication solution. Must be unique across your instance. | [optional]
**first_name** | Option<**String**> | The first name to assign to the user | [optional]
**last_name** | Option<**String**> | The last name to assign to the user | [optional]
**locale** | Option<**String**> | The locale to assign to the user (e.g., \"en-US\", \"fr-FR\") | [optional]
**primary_email_address_id** | Option<**String**> | The ID of the email address to set as primary. It must be verified, and present on the current user. | [optional]
**notify_primary_email_address_changed** | Option<**bool**> | If set to `true`, the user will be notified that their primary email address has changed. By default, no notification is sent. | [optional][default to false]
**primary_phone_number_id** | Option<**String**> | The ID of the phone number to set as primary. It must be verified, and present on the current user. | [optional]
**primary_web3_wallet_id** | Option<**String**> | The ID of the web3 wallets to set as primary. It must be verified, and present on the current user. | [optional]
**username** | Option<**String**> | The username to give to the user. It must be unique across your instance. | [optional]
**profile_image_id** | Option<**String**> | The ID of the image to set as the user's profile image | [optional]
**password** | Option<**String**> | The plaintext password to give the user. Must be at least 8 characters long, and cannot be in any list of hacked passwords. | [optional]
**password_digest** | Option<**String**> | In case you already have the password digests and not the passwords, you can use them for the newly created user via this property. The digests should be generated with one of the supported algorithms. The hashing algorithm can be specified using the `password_hasher` property. | [optional]
**password_hasher** | Option<**String**> | The hashing algorithm that was used to generate the password digest.  The algorithms we support at the moment are [`bcrypt`](https://en.wikipedia.org/wiki/Bcrypt), [`bcrypt_sha256_django`](https://docs.djangoproject.com/en/4.0/topics/auth/passwords/), [`md5`](https://en.wikipedia.org/wiki/MD5), `pbkdf2_sha1`, `pbkdf2_sha256`, [`pbkdf2_sha256_django`](https://docs.djangoproject.com/en/4.0/topics/auth/passwords/), `pbkdf2_sha512`, [`phpass`](https://www.openwall.com/phpass/), `md5_phpass`, [`scrypt_firebase`](https://firebaseopensource.com/projects/firebase/scrypt/), [`scrypt_werkzeug`](https://werkzeug.palletsprojects.com/en/3.0.x/utils/#werkzeug.security.generate_password_hash), [`sha256`](https://en.wikipedia.org/wiki/SHA-2), [`ldap_ssha`](https://www.openldap.org/faq/data/cache/347.html), the [`argon2`](https://argon2.online/) variants: `argon2i` and `argon2id`, `sha512_symfony`, the SHA-512 variant of the [Symfony](https://symfony.com/doc/current/security/passwords.html) legacy hasher, and `pbkdf2_sha512_hex`, a variant of `pbkdf2_sha512` that accepts hex-encoded salt and hash.  Each of the supported hashers expects the incoming digest to be in a particular format. See the [Clerk docs](https://clerk.com/docs/references/backend/user/create-user) for more information. | [optional]
**skip_password_checks** | Option<**bool**> | Set it to `true` if you're updating the user's password and want to skip any password policy settings check. This parameter can only be used when providing a `password`. | [optional]
**sign_out_of_other_sessions** | Option<**bool**> | Set to `true` to sign out the user from all their active sessions once their password is updated. This parameter can only be used when providing a `password`. | [optional]
**totp_secret** | Option<**String**> | In case TOTP is configured on the instance, you can provide the secret to enable it on the specific user without the need to reset it. | [optional]
**backup_codes** | Option<**Vec<String>**> | If Backup Codes are configured on the instance, you can provide them to enable it on the specific user without the need to reset them. | [optional]
**delete_self_enabled** | Option<**bool**> | If true, the user can delete themselves with the Frontend API. | [optional]
**create_organization_enabled** | Option<**bool**> | If true, the user can create organizations with the Frontend API. | [optional]
**legal_accepted_at** | Option<**String**> | A custom timestamp denoting _when_ the user accepted legal requirements, specified in RFC3339 format. | [optional]
**skip_legal_checks** | Option<**bool**> | When set to `true` all legal checks are skipped. | [optional]
**create_organizations_limit** | Option<**i32**> | The maximum number of organizations the user can create. 0 means unlimited. | [optional]
**created_at** | Option<**String**> | A custom date/time denoting _when_ the user signed up to the application. | [optional]
**bypass_client_trust** | Option<**bool**> | When set to `true`, the user will bypass Device Trust checks during sign-in. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



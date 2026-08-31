# Live validator integration tests

These tests exercise the Actix Web, Axum, Rocket, and Poem adapters with a real session JWT and the JWKS returned by a Clerk development instance. They send only a read-only `GET /jwks` request to Clerk; all framework requests run in-process and do not mutate Clerk data.

The tests are marked `#[ignore]`, so ordinary `cargo test` commands do not require credentials or make live network requests.

## Requirements

Use a development Clerk instance and provide:

- `CLERK_SECRET_KEY`: its secret key. The test harness refuses keys that do not begin with `sk_test_`.
- `CLERK_SESSION_TOKEN`: a fresh, raw session JWT for a signed-in development user. Do not include the `Bearer ` prefix.
- `CLERK_TEST_USER_ID`: that user's Clerk ID, such as `user_...`. The handlers return the validated JWT subject and the tests compare it with this value.

Session JWTs expire, so obtain a fresh token from your development application's normal Clerk session flow immediately before running the tests. A user password is neither needed nor requested.

Never paste credentials into an issue, commit them, or send them in chat. One local option is to create the gitignored `.env.validator-tests` file at the repository root:

```sh
CLERK_SECRET_KEY='sk_test_...'
CLERK_SESSION_TOKEN='eyJ...'
CLERK_TEST_USER_ID='user_...'
```

Load it into the current shell:

```sh
set -a
source .env.validator-tests
set +a
```

## Run one framework

```sh
cargo test --test live_actix --features actix -- --ignored
cargo test --test live_axum --features axum -- --ignored
cargo test --test live_rocket --features rocket -- --ignored
cargo test --test live_poem --features poem -- --ignored
```

## Run all frameworks

```sh
cargo test --all-features --tests -- --ignored --test-threads=1
```

Running sequentially makes failures easier to attribute and avoids simultaneous JWKS fetches.

Each framework test verifies:

- an unprotected or excluded route succeeds without credentials;
- a protected route rejects a missing token;
- a protected route rejects a malformed bearer token;
- a valid bearer token succeeds and exposes the expected JWT subject;
- the same token succeeds through the `__session` cookie when cookie validation is enabled.

If every authenticated request fails, first confirm that the session token is fresh, belongs to `CLERK_TEST_USER_ID`, and was issued by the same Clerk instance as `CLERK_SECRET_KEY`.

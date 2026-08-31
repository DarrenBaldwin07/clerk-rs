#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
spec="$root_dir/openapi/clerk-bapi-2026-05-12.yml"
generator_version="7.24.0"
output_dir="$(mktemp -d "${TMPDIR:-/tmp}/clerk-rs-openapi.XXXXXX")"
generated_dir="$output_dir/generated"
codegen_spec="$output_dir/clerk-bapi-2026-05-12.codegen.yml"

cleanup() {
	rm -rf -- "$output_dir"
}
trap cleanup EXIT

actual_version="$(openapi-generator-cli version)"
if [[ "$actual_version" != "$generator_version" ]]; then
	echo "expected OpenAPI Generator $generator_version, got $actual_version" >&2
	exit 1
fi

(
	cd "$root_dir/openapi"
	shasum -a 256 -c SHA256SUMS
)

node "$root_dir/scripts/prepare-openapi.mjs" "$spec" "$codegen_spec"

generator_log="$output_dir/generator.log"
if ! openapi-generator-cli generate \
	-q \
	-i "$codegen_spec" \
	-g rust \
	-o "$generated_dir" \
	--additional-properties=packageName=clerk-rs,packageVersion=0.5.0,useChrono=false,preferUnsignedInt=true,reqwestDefaultFeatures= \
	--global-property=apiTests=false,modelTests=false \
	>"$generator_log" 2>&1; then
	cat "$generator_log" >&2
	exit 1
fi

# Configuration is intentionally hand-written so the generated functions use
# clerk-rs's user agent, canonical API host, and pinned Clerk-API-Version header.
rsync -a --delete --exclude configuration.rs "$generated_dir/src/apis/" "$root_dir/src/apis/"
rsync -a --delete "$generated_dir/src/models/" "$root_dir/src/models/"
rsync -a --delete "$generated_dir/docs/" "$root_dir/docs/"
cp "$generated_dir/.openapi-generator/VERSION" "$root_dir/.openapi-generator/VERSION"

# The Markdown templates currently emit trailing spaces in table rows.
perl -pi -e 's/[ \t]+$//' "$root_dir"/docs/*.md

# OpenAPI Generator normalizes `frontendApi` and `frontend_api` to the same
# local identifier. Apply the small reviewed correction for that collision.
patch -d "$root_dir" -p1 --forward --silent < "$root_dir/openapi/generated.patch"

# Format generated modules directly. Cargo fmt's module discovery can skip
# freshly replaced generated files while this script's temporary crate exists.
rustfmt --edition 2021 "$root_dir"/src/apis/*.rs "$root_dir"/src/models/*.rs

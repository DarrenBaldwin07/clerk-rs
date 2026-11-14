#!/usr/bin/env node

import fs from "node:fs";

const [inputPath, outputPath] = process.argv.slice(2);
if (!inputPath || !outputPath) {
	throw new Error("usage: prepare-openapi.mjs <input-spec> <output-spec>");
}

// Clerk's dated specification uses unformatted OpenAPI integers for a few
// millisecond timestamps. OpenAPI Generator maps those to i32, even though
// Clerk returns values far larger than i32::MAX. Keep the vendored upstream
// file byte-for-byte intact and add `format: int64` only in the temporary spec
// used for code generation.
const timestampProperties = new Set(["expire_at", "last_active_at"]);
const timestampParameters = new Set([
	"last_active_at_before",
	"last_active_at_after",
	"last_active_at_since",
	"created_at_before",
	"created_at_after",
	"last_sign_in_at_before",
	"last_sign_in_at_after",
]);

const lines = fs.readFileSync(inputPath, "utf8").split("\n");
const blocks = [];

for (let index = 0; index < lines.length; index += 1) {
	const property = lines[index].match(/^(\s+)([a-z0-9_]+):\s*$/);
	const parameter = lines[index].match(/^(\s+)- name: ([a-z0-9_]+)\s*$/);
	const isTimestampProperty = property && timestampProperties.has(property[2]);
	const isTimestampParameter = parameter && timestampParameters.has(parameter[2]);
	if (!isTimestampProperty && !isTimestampParameter) continue;

	const baseIndent = (property ?? parameter)[1].length;
	let end = index + 1;
	while (end < lines.length) {
		const line = lines[end];
		if (line.trim() !== "" && line.match(/^\s*/)[0].length <= baseIndent) break;
		end += 1;
	}

	blocks.push({ start: index, end });
}

const insertions = [];
for (const { start, end } of blocks) {
	for (let index = start + 1; index < end; index += 1) {
		const integer = lines[index].match(/^(\s+)type: integer\s*$/);
		if (!integer) continue;

		const typeIndent = integer[1];
		const alreadyInt64 = lines.slice(index + 1, end).some((line) => line === `${typeIndent}format: int64`);
		if (!alreadyInt64) insertions.push({ index: index + 1, line: `${typeIndent}format: int64` });
		break;
	}
}

const expectedInsertions = 30;
if (insertions.length !== expectedInsertions) {
	throw new Error(
		`expected ${expectedInsertions} unformatted millisecond timestamps, found ${insertions.length}; review the Clerk spec and this normalization script`,
	);
}

for (const insertion of insertions.reverse()) {
	lines.splice(insertion.index, 0, insertion.line);
}

// A few responses are an `anyOf` of object schemas that cannot overlap. OpenAPI
// Generator's Rust templates flatten `anyOf` into a single struct requiring
// every member's properties, which no real response satisfies. `oneOf` produces
// an untagged enum instead, so rewrite `anyOf` to `oneOf` for those operations
// in the temporary spec only. The members are inline schemas, so also give each
// one a `title`, which the generator uses as its model and enum variant name.
const exclusiveAnyOfPaths = {
	"/oauth_applications/access_tokens/verify": ["OAuthAccessTokenDetails", "ExpiredJwtAccessToken"],
};
let exclusiveAnyOfRewrites = 0;

for (const [path, memberTitles] of Object.entries(exclusiveAnyOfPaths)) {
	const start = lines.indexOf(`  ${path}:`);
	if (start === -1) {
		throw new Error(`expected path ${path} in the Clerk spec; review this normalization script`);
	}

	let end = start + 1;
	while (end < lines.length && !lines[end].match(/^ {0,2}\S/)) end += 1;

	const anyOfLines = [];
	for (let index = start + 1; index < end; index += 1) {
		if (lines[index].match(/^\s+anyOf:\s*$/)) anyOfLines.push(index);
	}
	if (anyOfLines.length !== 1) {
		throw new Error(
			`expected 1 anyOf schema under ${path}, found ${anyOfLines.length}; review the Clerk spec and this normalization script`,
		);
	}

	const anyOfIndex = anyOfLines[0];
	const memberIndent = `${lines[anyOfIndex].match(/^\s*/)[0]}  `;
	const members = [];
	for (let index = anyOfIndex + 1; index < end; index += 1) {
		if (lines[index].trim() !== "" && lines[index].match(/^\s*/)[0].length < memberIndent.length) break;
		if (lines[index] === `${memberIndent}- type: object`) members.push(index);
	}
	if (members.length !== memberTitles.length) {
		throw new Error(
			`expected ${memberTitles.length} anyOf members under ${path}, found ${members.length}; review the Clerk spec and this normalization script`,
		);
	}

	lines[anyOfIndex] = lines[anyOfIndex].replace("anyOf:", "oneOf:");
	for (let member = members.length - 1; member >= 0; member -= 1) {
		lines.splice(members[member] + 1, 0, `${memberIndent}  title: ${memberTitles[member]}`);
	}
	exclusiveAnyOfRewrites += 1;
}

fs.writeFileSync(outputPath, lines.join("\n"));
process.stdout.write(`normalized ${insertions.length} millisecond timestamp schemas to int64\n`);
process.stdout.write(`normalized ${exclusiveAnyOfRewrites} mutually exclusive anyOf schemas to oneOf\n`);

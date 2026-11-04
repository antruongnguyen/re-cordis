# Create pinned source dispositions

Source pin: `35fdc969e4b6c48c48ed44717c7a018324e4dd88`.
Source file: `local/cordis/packages/create/tests/scaffold.spec.ts`.
All 14 `it` declarations are listed below. This is a source disposition, not
14 executed Rust/TypeScript equivalents. No TypeScript Create test or downloader
is claimed as executed by the offline Rust acceptance.

The implementation reference is `src/index.ts` (prepare, scaffold,
writePackageJson, initGit, install, start); `src/bin.ts` selects the upstream
registry template. The test fixtures construct package archives inline with
`archive()` and serve them through `startRegistry()`; the selected Rust template
is a compile-time bundled file set and has no remote/template-input path.

| TS line | Declaration | Classification | Rust scope / evidence |
|---|---|---|---|
| 134 | scaffolds the default template to completion | Cargo equivalent | Fixed basic files and actual renamed Cargo package; external Cargo build/test/run through `tests/docs/check.py`, plus scaffold integration tests. No remote trace equivalence claimed. |
| 143 | reports a registry without a scheme | Unselected remote transport | No registry input or URL parser. CLI rejects unsupported flags; registry validation is not an executed Rust case. |
| 149 | reports an unknown dist-tag | Unselected remote transport | No template version/tag selection. |
| 156 | reports a failed tarball request with its url | Unselected remote transport | No HTTP/download path. |
| 163 | reports a template without package.json | Portable failure purpose; Cargo representation | Invalid required local checkout/package input is rejected before committing a project; exact bundled file set and real Cargo metadata/build validate the generated manifest. There is no external template or package.json parser, so this does not reproduce its malformed-archive fixture. |
| 171 | reports invalid registry json | Unselected remote transport | No registry metadata JSON. |
| 178 | reports an invalid tarball | Unselected remote transport | No archive extraction. |
| 186 | reports a tarball response without a body | Unselected remote transport | No streamed HTTP body. |
| 193 | reports a registry that never sends headers | Unselected remote transport | No HTTP/header deadline. |
| 201 | reports a registry body timeout as a download failure | Unselected remote transport | No remote metadata body. |
| 209 | reports a tarball body timeout | Unselected remote transport | No archive stream/deadline. |
| 216 | distinguishes a dropped download from an invalid archive | Unselected remote transport | No transport/archive distinction to implement. |
| 228 | fails when automatic yarn setup cannot download its release | npm/Yarn-specific exclusion | No package-manager detection, staging, cache or automatic installation. |
| 238 | reports a pinned yarn download timeout without changing the rc | npm/Yarn-specific exclusion | No yarnPath/.yarnrc configuration or binary download. |

Totals: one Cargo-equivalent purpose, one portable-failure/Cargo representation,
ten unselected remote-transport declarations, two npm/Yarn exclusions.
None of the 12 exclusions is counted as a passing test. Rust filesystem
validation, non-destructive commit and generated lifecycle regressions are
additional native requirements, not invented upstream declarations.

The upstream `prepare()` can delete an existing destination after confirmation
or force/yes; the offline Rust policy deliberately refuses nonempty destinations.
Git/init and install/launch are omitted entirely, with readable Cargo commands
left to the user. The supported transaction and concurrency boundaries are in
[the Create contract](create-contract.md).

The [initial independent review](create-review.md) verified this disposition;
[scoped re-review](create-rereview.md) preserves it after CR1/CR2 corrections.
Final executed counts and scope are in [acceptance](create-acceptance.md).

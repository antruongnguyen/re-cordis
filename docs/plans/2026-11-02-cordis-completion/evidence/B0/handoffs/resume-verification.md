# Integrated B0 recovery verification

Executor: gpt-6.1-sol/high. Run in the integration checkout
`/Users/I756434/Projects/private/cordis-rust`, branch feat/cordis-completion.
The correction is integrated at `59be2db`; matrix report re-review at `b87c1b9`.
Read AGENTS.md, the corrected ledger-inputs/report-decorator-reflect.md and
../matrix-rereview.md. Own only ../resume-validation.md. Do not modify the
fragment/report/checkers/progress/source, commit, spawn agents or push.

Repeat the fragment's bounded structural validation against the integrated
files. Adapt only its changed-files assertion for integration: compare the
two owned files at `59be2db` against `25329ad`, since additional tracking and
review commits are expected. Confirm exactly seven routing changes and all
other JSON data equal; five declarations, five variants and 23 obligations.

Check current executable inputs against all 71 baseline hashes and the pinned
clean TypeScript checkout, and verify original matrix artifacts unchanged.
Check links in the two corrected files, matrix-rereview.md and these recovery
handoffs with the existing docs checker's link helper (inspect its API first).
No Cargo/docs-example/parity rerun is justified unless executable inputs differ.
Use standard-library checks, record exact commands, Python version, UTC times,
integration HEAD, actual exits and compact outputs in resume-validation.md.
Run git diff --check. Report scope, results, file path and any failure.

# Current review: services

Reviewer: gpt-6-astra/high, fresh context. Follow family-review.md beside
this file for assignment **services**, implementation commit
d07d3eec2df3979d5413a0ed7477feebe70efe4c. Create an isolated managed
worktree there from /Users/I756434/Projects/private/cordis-rust. Own only
evidence/B0/services-review.md in the completion-plan tree.

Inputs are ledger-inputs/service.json, service-dispose.json and
report-services.md. Claims: 25 declarations, 35 variants, 81 obligations,
61 source-mapped tests, 15 Rust representations, five evidence gaps and
19 Rust test identities with both-toolchain result/log joins.

The draft intentionally omits source line 125 from the provider, root,
provider first and consumer first variants of core/service-dispose.spec.ts:95,
retaining it only for restart. Independently confirm the source guard and
whether this is a justified correction to the old inventory. Specify explicit
provenance metadata for those four original IDs in final integration; do not
silently lose them or count inapplicable assertions as gaps or passes.

Apply the common semantic/reference review, including all meaningful helper
paths, exact retained-provider and initialization/callback boundaries, and
the stated gap witnesses. Do not reopen accepted earlier fragments or rerun
executable suites. Commit only your review and return a readable verdict.

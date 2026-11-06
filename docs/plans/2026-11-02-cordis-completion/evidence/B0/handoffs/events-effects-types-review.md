# Current review: events-effects-types

Reviewer: gpt-6-astra/high, fresh context. Follow family-review.md beside
this file for assignment **events-effects-types**, implementation commit
58117f8d021bcc9e7edd0322d4faebc357dfb27a (initial data d729b8a plus the
verified Rustdoc-metadata correction). Create an isolated managed
worktree there from /Users/I756434/Projects/private/cordis-rust. Own only
evidence/B0/events-effects-types-review.md in the completion-plan tree.

Inputs under B0: ledger-inputs/events-rest.json, dispose.json, event-types.json
and **report-events-effects-types.md at B0 root**. The report is at B0 root,
not inside ledger-inputs; this is the four-file assignment's actual committed
report location. Claims: 30 declarations / 30 variants; 60 remaining-event
obligations, 39 dispose obligations and six type obligations; 87 Rust
representations, 6 mapped tests, 12 evidence gaps, 24 Rust test/Rustdoc items.

Check all newly added obligations as well as retained original IDs and every
dispatch/effect/type assertion branch. First-two events are already accepted
at 11d84b1 and are excluded. Preserve the typed event/continuation/effect
representations from prior M2–M6 decisions. Verify compile-fail versus
runnable Rustdoc joins for the six type expressions. Apply the common review
instructions, record actual checks, commit only your review and return a
readable worktree/commit/verdict handoff.

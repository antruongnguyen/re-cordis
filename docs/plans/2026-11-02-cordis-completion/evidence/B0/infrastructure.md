# B0 worker execution blocker

Date: 2026-11-02. This record describes observed agent infrastructure failures,
not Rust failures or evidence of a global provider outage. Orchestrator evidence
and Git operations remained available. No alternative model was substituted.

## Requested settings and observed attempts

All execution workers were spawned with model gpt-6.1-sol,
reasoning_effort high, fork_turns none. Planning/review used gpt-6-astra/high.

| Context | Outcome | Recovery tried |
|---|---|---|
| b0_ledger | response stream disconnected before completion | Same-context resume retained an inventory checkpoint, then failed HTTP 400 at 2026-11-02T08:40:28Z |
| b0_ledger_extra | stream disconnected on original turn and resumed turn | Same-model fresh bounded context |
| b0_matrix | all required commands ran; response disconnected during report assembly | Same-context report-only resume failed HTTP 400 at 2026-11-02T08:40:41Z; full command artifacts preserved |
| b0_ledger_extra_retry | fresh bounded decorator/reflect assignment completed | Its five-record fragment is saved; success did not establish general recovery |
| b0_events | fresh bounded events-family assignment failed HTTP 400 at 2026-11-02T08:49:25Z | Saved nineteen-record initial inventory; no repeated equivalent retry scheduled |
| b0_matrix_report | bounded fresh report task completed; summary/report consistency checks recorded | Report saved at 25dcde8; no full-matrix rerun |
| b0_ledger_extra_retry follow-up | received the correction request as opaque gAAAAA-prefixed ciphertext, requested readable scope and ended without changes | Durable handoffs/fragment-fix.md retained; FR-1 remains open |

The three HTTP 400 failures had errorCode UPSTREAM_LLM_ERROR, provider openai,
and error code invalid_encrypted_content: an encrypted reasoning item could
not be verified because it could not be decrypted or parsed. Returned item IDs:

- b0_ledger: rs_058c962ffe82ae06016ae84a8260fc8197aa11dc0e35b63a48
- b0_matrix: rs_0eec2e2f5e3f496c016ae84a503df081978c02eb898fb4155b
- b0_events: rs_058644985debf2c7016ae84e0f13288196b8939d232cf34e51

A matrix reviewer initially received an unreadable encrypted-looking handoff.
A durable file-based handoff allowed its independent review to finish. That
review accepted the command matrix, but it does not complete the case ledger.

## Retained state and boundary

All 53 indexed command logs have complete metadata and exit zero. Two separate
summary-attempt logs have incomplete execution metadata; their limits are
spelled out in matrix-review.md and must not be retroactively invented.
The original, resumed and fresh-context worker failures are unrelated to those
Rust/documentation/parity results. Source hashes and the pinned TS checkout
remained unchanged. The events fragment is inventory-only; decorator/reflection
has meaningful mappings but must pass its independent review before integration.

The attempted remedies were same-agent resume, smaller incremental outputs,
a fresh same-model context, and short file-based handoffs. Fresh-context HTTP
400 and unreadable correction transport show these remedies did not reliably
remove the blocker. B0 is blocked; its hard-dependent packages cannot be
started as though its audit were complete. Independent report housekeeping
and fragment review finished. The matrix report is saved with complete new
validation metadata and preserved old attempts; fragment review requires only
the FR-1 scheduling correction, which could not be executed. All workers ended.

## Safe recovery

Read ../../progress.md, inspect current Git status/history/worktrees and all
saved evidence first. Do not resume the known corrupted contexts. Confirm that
a fresh worker with the exact requested model/effort can complete a bounded
task, then continue the remaining B0 families from the ownership table. Preserve
partial files and fill the accepted schema rather than treating placeholders
as reviewed missing-evidence dispositions. Merge 144 declarations/156 variants
and six type obligations, run the ledger checks, obtain independent review,
and only then start T0. Rerun the matrix when executable inputs change or the
recorded hashes no longer establish applicability, not solely due to a new
tracking commit. No background continuation is promised after the session ends.


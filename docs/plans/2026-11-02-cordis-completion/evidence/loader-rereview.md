# Loader final scoped re-review

Date: 2026-11-03 (Asia/Ho_Chi_Minh).
Reviewer: independent gpt-6-astra/high.
Reviewed implementation: `101f37b`.
Verdict: **all F1–F8 addressed; specification and code quality approved for the
agreed L0–L5/G0–G1 Rust scope and documented bounds**.

This closes [the prior broad review](loader-review.md). The reviewer reused
the six accepted correction verdicts and reviewed the final F4/F5 delta;
no new broad audit or package-wide test rerun was performed.

| Finding | Verdict and current source evidence |
|---|---|
| F1 | Addressed. Revision/liveness checks stop stale reconciliation across child callbacks and membership replacement; src/lib.rs:1291. |
| F2 | Addressed. Condition/intercept groups use lifecycle rebuild, and the structural fast path excludes them; src/lib.rs:1624. Both Active-to-Failed-to-explicit-recovery regressions are retained. |
| F3 | Addressed. Shallow validation and incremental source identity claims preserve valid siblings and locate collisions; src/lib.rs:2389. |
| F4 | Addressed. Separate update barriers and reentrant reservations enforce the 1024-operation bound; src/lib.rs:1593. Settlement, subtree removal and disposal join retained work; bounded first-unobserved error retention and acknowledgement match the documented policy; src/lib.rs:2778. |
| F5 | Addressed, including the scoped reentry follow-up. Non-group replacement removes actual children; rebuild work is published before commit callbacks; detached descendant cleanup precedes replacement apply; src/lib.rs:1569, 1649 and 1452. |
| F6 | Addressed. Existing ancestor IDs enter preflight before mutation or commit; src/lib.rs:1537. |
| F7 | Addressed. Obsolete source ownership is pruned; weak named realms have bounded live definitions and documented identity lifetime; src/lib.rs:943 and 2039. |
| F8 | Addressed. Early malformed nested group configuration carries its actual config path through source/span decoration; src/lib.rs:2061. |

Source evidence refers to [loader lib.rs](../../../../crates/re-cordis-loader/src/lib.rs)
at the reviewed implementation. Regression coverage is retained in
[final_wave.rs](../../../../crates/re-cordis-loader/tests/final_wave.rs).

## Focused external reproduction

The initial same-parent commit-callback probe reproduced the remaining F5
membership defect and premature completion during gated child cleanup. On the
corrected implementation the gated public-API reproduction exits successfully:

```text
update_completed_before_child_cleanup=false
entries_before_release=[("parent", Unloading)]
entries_after_settle=[("parent", Active)]
```

The in-tree regression also asserts that replacement applies remain blocked
before the cleanup gate, then only the latest requested value applies.

No new Critical/Important defect or additional out-of-scope finding was
confirmed in the correction delta. Source/test hashes match the supplied
frozen fingerprints. No core changes or new production dependencies were
introduced. The parent’s explicit stable 1.97.1/MSRV 1.85.0 matrices and paired
trace results supply final verification; see [acceptance](loader-acceptance.md).

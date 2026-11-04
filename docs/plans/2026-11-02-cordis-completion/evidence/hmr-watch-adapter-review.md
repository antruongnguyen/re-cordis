# HMR H1 watcher and H3 adapter independent final review

Date: 2026-11-04 (Asia/Ho_Chi_Minh).
Reviewed source: `77b50b5bdca92ab32d8acc4a9f2d58753c32e3ba`.
H3 delta: `e44754e..77b50b5`; original H1 checkpoint: `56de5f4`.
H0 accepted design: `5065adc`; accepted H2: `0e8d306`.

**Final independent specification/quality/integration review: accepted at
55a55d1.** The sole H1-R4 finding is corrected; no actionable H1/H3 source or
frozen-documentation finding remains. Accepted H2 carries forward unchanged.
This closes the independent watcher/adapter review gate. The parent-owned shared
workspace/M0/docs/examples/parity matrix remains the separate final delivery gate.

The original source assessment at 77b50b5 required the one bounded H1 correction
below and found no actionable H3 adapter defect. Its RED evidence is retained.

## Scope and carried assessment

Read AGENTS.md, HMR/shared acceptance plans, combined/watch contracts, H0 review,
H1/H3 implementation checkpoints and H2 accepted review. CodeGraph supplied
source/call paths; RustRover VFS filled omitted ranges. Reviewed the complete
watcher/path implementation, H1 test oracles, all ten H3 tests, offline example,
manifests and H3 source diff. The loader delta after accepted H2 is only rustfmt
whitespace; its replacement, lifecycle, diagnostics and Include forwarding
assessment carries forward. No core or prerequisite audit was reopened.

The reviewer changed only this report and ignored `target/hmr-final-review/`
probes/logs. No production code, committed tests, Git index, branch or commit
was changed. The frozen documentation pass and final correction review are recorded below.

## H1-R4 — retry a read failure lost at ingress

Priority: P2. `crates/re-cordis-hmr/src/watch.rs:1200–1207`.

The producer stores `entry.2 = Some(key)` before sending its first read error,
then ignores `Shared::send` returning `RescanRequested`. A full ingress queue
loses the error, but every later sample with the same kind/message remains
suppressed. Draining ingress and retaining ample event-history capacity cannot
make the persistent fault observable. Overflow Rescan marks callbacks dirty;
it does not reset the producer error-announcement marker.

The independent public probe uses `queue_capacity = 1`, two oversized files,
and a local-host hold bounded by a two-second overflow condition. After ingress
drains and samples continue, both toolchains report one Read failure path instead
of two: `overflow=1`, `events_evicted=0`, producer joined. This differs from
permitted eviction of a failure already published in bounded event history.

Correction criterion: retry unannounced errors after failed enqueue; mark the
error announced only after actual admission. Preserve coalescing after Queued,
generation suppression, bounds and joined shutdown; no unbounded backlog.

Reproducer: `target/hmr-final-review/probe/src/lib.rs`,
`overflow_does_not_permanently_hide_read_failures`.
Failure logs: `overflow-{stable,1.85.0}.log`; commands: `probe-results.json`.
Initial scratch builds emitted harmless unused test-import warnings; cfg(test)
now removes them. The assertion failure is independent of those warnings.

## Other source conclusions

Admission precedes producer/registration mutation. Local handles stay on the
original host; only owned paths, generations, I/O errors and bounded synchronized
state cross threads. Generation checks suppress stale work. Exact stop, retiring
quota, one running callback and one coalesced follow-up have explicit lifetime
tracking. Callback/error destruction occurs outside mutable framework borrows.
Self/ancestor disposal is refused before stop; the lifecycle cleanup joins the
producer JoinHandle. Blocked OS reads retain their documented delay caveat.

H3 stores its already-admitted transformed IncludeTask on the entry before
returning the cancelable callback future. The receiver observes a shared clone;
taking stored work is the single reporting point. Cleanup awaits the same work
without new Include admission. Reclamation waits for completion observation and
callback destruction. Returned IncludeErrors and every Report failure retain
original typed objects. The adapter neither disposes Include nor discovers paths.

The held-cleanup OriginalFailure(17) remains correctly observable through Runtime
diagnostics, as the direct Include control proves. The four held adapter cases
also assert both real Report patch errors exactly once with typed IncludeError,
source file, patch indices and spans. Their actual gated release, pending barrier,
quota, no-follow-up and no-spurious-disposed-error assertions remain intact.
Parse recovery preserves the serde_json cause. No accepted diagnostic channel
needs rerouting.

The H1 removal-oracle change waits for both missing bytes and Removed, preventing
an older queued callback from satisfying the wait early. It preserves resource
and sticky-path assertions. The offline example uses the same Include-managed
entry for watched config, linked factory replacement and later refresh, reports
source RestartRequired, and joins teardown.

## Fresh independent checks at 77b50b5

All five cargo/rustc/rustdoc/cargo-clippy/clippy-driver paths were resolved with
`rustup which --toolchain stable|1.85.0`; matching toolchain-bin PATH,
RUSTUP_TOOLCHAIN, RUSTC and RUSTDOC were selected. Stable rustc: 1.97.1; MSRV:
1.85.0. Tool paths/versions: `tools-{stable,1.85.0}.json`. Input hashes:
`reviewed-source-77b50b5.json`. The isolated probe is locked after first offline
resolution.

| Fresh check | stable | 1.85.0 |
|---|---:|---:|
| Overflow read-error public probe | 1 failed (H1-R4) | 1 failed (H1-R4) |
| Independent lifecycle/admission probes | 4 passed | 4 passed |
| HMR all-feature locked/offline tests | 54 integration + 1 Rustdoc passed | 54 integration + 1 Rustdoc passed |

The four passing probes cover missing/wrong/inactive admission without new
registration; async callback self/ancestor join refusal followed by another valid
callback; watcher Drop retaining admitted Include work until ancestor completion;
and dropped ancestor waiter without revoked cleanup. Both Drop cases assert
pending completion, actual gated release, suppressed third config, empty Registry
and no spurious Runtime failures. Logs: `lifecycle-probe-*`, `hmr-all-*`;
commands/exit codes: `focused-results.json`.

Existing focused matrices remain worker-attributed. Parent owns final shared
workspace/M0/docs/examples/parity execution. Runtime smoke here is aarch64 macOS;
Linux/Windows runtime behavior, Node import/cache trace parity, native swapping
and create/R0/R1 are not claimed.


## Frozen documentation consistency at 3f5e562

Reviewed all 23 files in `77b50b5..3f5e562`: paired HMR guide/API, Include/loader
cross-links and example indexes, project/status documents (including the parent progress update), decision/limitation
records, llms.txt and the full HMR source disposition map. No documentation
finding remains in this frozen diff. Current claims correctly separate implemented
behavior, accepted H2, pending H1/H3 acceptance, and unsupported Node/platform
mechanisms. EN/VI wording agrees on cancellation versus completion, retained
IncludeTask, quota, both error channels, explicit paths and already-linked code.
The two runnable guide blocks (Rust and shell) match exactly between languages.

A mechanical declaration check against the actual TypeScript pin found all
49 index.spec.ts and four path.spec.ts declarations exactly once, with matching
names and line numbers. Both ledger tables have five columns. The source map
names all ten H3 tests through I-01–I-06, carries accepted H2, and explicitly
avoids claiming Node execution, unchanged-host nested discovery or Windows/Linux
runtime parity. Rust oracle references agree with the reviewed tests and their
resource assertions. This consistency check did not rerun the TypeScript suite
or independently repeat every previously accepted B0/H2 trace.

llms.txt is 8103 bytes. The 23 reviewed files match committed bytes at `3f5e562`;
SHA-256 values and the verified upstream pin are in
`target/hmr-final-review/reviewed-docs.json`. Full documentation checker execution
remains parent-attributed; its reported stable/1.85.0 pass is not counted as a
reviewer rerun. At that documentation checkpoint H1-R4 remained pending; the correction is closed below.


## H1-R4 correction acceptance at 55a55d1

Reviewed `3f5e562..55a55d1`: the producer now saves its announced-error signature
only after `Shared::send` returns `Ok(Queued)`. `RescanRequested` and Closed do
not announce it. An error still present on a later sample is retried after queue
capacity returns; an already-queued identical error remains coalesced. Successful
samples still reset the signature. Path-generation checks, cancellation, queue
capacity, retained sample bounds and producer shutdown are unchanged. No task,
backlog, dependency, or local-host state was added to the producer.

The new regression forces two distinct oversized paths to compete for one queue
slot, resumes the host, verifies both actual io::Error objects/kinds/messages,
then verifies no additional unchanged failures. It records zero history eviction
and joins producer disposal before assertions, checking zero queued messages and
resident registrations. It therefore protects both retry and coalescing while
observing resource release. The correction changelog, source-map supplement,
progress entry and implementation evidence match this narrow behavior. Paired
API wording from the accepted documentation checkpoint needs no semantic change.

Fresh independent commands, using the same explicitly resolved tool environments:

```sh
cargo test --manifest-path target/hmr-final-review/probe/Cargo.toml --locked --offline -- --nocapture
cargo test -p re-cordis-hmr --all-features --locked --offline --test filesystem overflow_retries_original_read_failures_then_coalesces_delivery -- --nocapture
```

| Final fresh check | stable 1.97.1 | 1.85.0 |
|---|---:|---:|
| Unchanged public probe suite, including H1-R4 and four lifecycle controls | 5 passed | 5 passed |
| Delivered retry/coalescing/resource regression | 1 passed | 1 passed |
| `git diff --check 3f5e562..55a55d1` | Pass | Same committed diff |

The original failing overflow function and full five-probe file are unchanged
from the earlier snapshot. It now prints both paths with `overflow=1`,
`evicted=0`, and `joined=true` on each toolchain. Full logs and command exit codes
are in `corrected-public-probes-*`, `corrected-regression-*` and
`corrected-results.json` under `target/hmr-final-review/`.

Source hashes matched the freeze before and after both runs:

| File | SHA-256 |
|---|---|
| HMR src/watch.rs | `a24dbdbc5a991ac45c3a767717c38d35f62f49f7a0706be6537fdd8bc48e6936` |
| HMR tests/filesystem.rs | `dd3549be7725c9d24fb66e9a4657bf3f1608bd40be40297786b3028112b09e10` |
| Unchanged independent probe src/lib.rs | `a9b4ff89d4af1961031ca1865859af2826a8b3d927f6d20794d0fd5422a5e2e4` |

The full corrected HMR 55 integration + 1 Rustdoc all-feature / 44 + 1 default
matrix, strict check/Clippy/Rustdoc and example execution remain attributed to
[the correction worker](hmr-overflow-correction.md). This reviewer freshly reran
the complete HMR all-feature suite at the original source freeze and the focused
changed behavior at the final correction, rather than repeating unchanged suites.
No unsupported platform or Node HMR parity claim is added by this acceptance.

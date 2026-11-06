# GitHub Actions refresh audit — 2026-11-05

This audit covers every `uses:` entry in `.github/workflows/ci.yml`,
`documentation.yml` and `pages.yml`, starting from commit
`abfa0f3092fd8b06765d3c004fbf130793b69fe5`. It refreshes action implementations
and replaces the retired macOS runner without changing the Rust contracts,
validation commands, permissions or manual Pages opt-in.

## Live source checks

Queried the official GitHub repositories on 2026-11-05, around 06:28 UTC.
For versioned actions, GitHub's `GET /repos/{owner}/{repo}/releases/latest`
selected the latest published stable release; the corresponding
`GET /repos/{owner}/{repo}/git/ref/tags/{tag}` was resolved to its complete
commit SHA, dereferencing annotated tags when necessary. Each selected commit's
`action.yml` and `README.md` were then read directly. The workflow comments
show exact release versions, while the executable references use immutable
40-character commit SHAs.

| Action | Selected release | Full commit SHA | Official sources |
| --- | --- | --- | --- |
| actions/checkout | v7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` | [Release](https://github.com/actions/checkout/releases/tag/v7.0.1), [manifest](https://github.com/actions/checkout/blob/3d3c42e5aac5ba805825da76410c181273ba90b1/action.yml), [documentation](https://github.com/actions/checkout/blob/3d3c42e5aac5ba805825da76410c181273ba90b1/README.md) |
| actions/setup-python | v7.1.0 | `6d4fd22b20465ccc0f7354476588e8df7c9ca0e0` | [Release](https://github.com/actions/setup-python/releases/tag/v7.1.0), [manifest](https://github.com/actions/setup-python/blob/6d4fd22b20465ccc0f7354476588e8df7c9ca0e0/action.yml), [documentation](https://github.com/actions/setup-python/blob/6d4fd22b20465ccc0f7354476588e8df7c9ca0e0/README.md) |
| actions/setup-node | v7.1.0 | `949feb2413d6458794dcd2491c4babbbce0c15c1` | [Release](https://github.com/actions/setup-node/releases/tag/v7.1.0), [manifest](https://github.com/actions/setup-node/blob/949feb2413d6458794dcd2491c4babbbce0c15c1/action.yml), [documentation](https://github.com/actions/setup-node/blob/949feb2413d6458794dcd2491c4babbbce0c15c1/README.md) |
| actions/upload-artifact | v7.0.2 | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` | [Release](https://github.com/actions/upload-artifact/releases/tag/v7.0.2), [manifest](https://github.com/actions/upload-artifact/blob/cf430e030ddbb5b0abf93d22962f4752f3646cd9/action.yml), [documentation](https://github.com/actions/upload-artifact/blob/cf430e030ddbb5b0abf93d22962f4752f3646cd9/README.md) |
| actions/download-artifact | v8.0.2 | `9000827ccba6bdab643e8b6fd33ac0654aef8333` | [Release](https://github.com/actions/download-artifact/releases/tag/v8.0.2), [manifest](https://github.com/actions/download-artifact/blob/9000827ccba6bdab643e8b6fd33ac0654aef8333/action.yml), [documentation](https://github.com/actions/download-artifact/blob/9000827ccba6bdab643e8b6fd33ac0654aef8333/README.md) |
| actions/configure-pages | v6.0.0 | `45bfe0192ca1faeb007ade9deae92b16b8254a0d` | [Release](https://github.com/actions/configure-pages/releases/tag/v6.0.0), [manifest](https://github.com/actions/configure-pages/blob/45bfe0192ca1faeb007ade9deae92b16b8254a0d/action.yml), [documentation](https://github.com/actions/configure-pages/blob/45bfe0192ca1faeb007ade9deae92b16b8254a0d/README.md) |
| actions/upload-pages-artifact | v5.0.0 | `fc324d3547104276b827a68afc52ff2a11cc49c9` | [Release](https://github.com/actions/upload-pages-artifact/releases/tag/v5.0.0), [manifest](https://github.com/actions/upload-pages-artifact/blob/fc324d3547104276b827a68afc52ff2a11cc49c9/action.yml), [documentation](https://github.com/actions/upload-pages-artifact/blob/fc324d3547104276b827a68afc52ff2a11cc49c9/README.md) |
| actions/deploy-pages | v5.0.1 | `368f82528645a54fb793d4d04e342629a3f51346` | [Release](https://github.com/actions/deploy-pages/releases/tag/v5.0.1), [manifest](https://github.com/actions/deploy-pages/blob/368f82528645a54fb793d4d04e342629a3f51346/action.yml), [documentation](https://github.com/actions/deploy-pages/blob/368f82528645a54fb793d4d04e342629a3f51346/README.md) |
| oven-sh/setup-bun | v2.2.0 | `0c5077e51419868618aeaa5fe8019c62421857d6` | [Release](https://github.com/oven-sh/setup-bun/releases/tag/v2.2.0), [manifest](https://github.com/oven-sh/setup-bun/blob/0c5077e51419868618aeaa5fe8019c62421857d6/action.yml), [documentation](https://github.com/oven-sh/setup-bun/blob/0c5077e51419868618aeaa5fe8019c62421857d6/README.md) |
| dtolnay/rust-toolchain | Maintained master snapshot, 2026-11-05 | `683b588c487a69d01f6c6ddf40f036ff682cc2c8` | [Master ref API](https://api.github.com/repos/dtolnay/rust-toolchain/git/ref/heads/master), [commit](https://github.com/dtolnay/rust-toolchain/commit/683b588c487a69d01f6c6ddf40f036ff682cc2c8), [manifest](https://github.com/dtolnay/rust-toolchain/blob/683b588c487a69d01f6c6ddf40f036ff682cc2c8/action.yml), [documentation](https://github.com/dtolnay/rust-toolchain/blob/683b588c487a69d01f6c6ddf40f036ff682cc2c8/README.md) |

## Toolchain selection convention

`dtolnay/rust-toolchain` documents toolchain-selecting rolling branches rather
than a semantic-versioned implementation release cadence. Its
[latest release API](https://api.github.com/repos/dtolnay/rust-toolchain/releases/latest)
reports `v1`, published 2022-07-15; the current tag resolved to the same SHA as
master, whose [latest commit](https://github.com/dtolnay/rust-toolchain/commit/683b588c487a69d01f6c6ddf40f036ff682cc2c8)
is dated 2026-10-16. No semantic version was invented for the implementation pin.

The upstream README explicitly requires pinned SHAs to belong to master's
history: commits generated for toolchain-selecting branches can eventually be
garbage-collected. The queried `stable` branch was
`b725eda525362e7cf34af9a7243cac1e3f01a7ec`, with `toolchain: stable` supplied
as a manifest default; it was inspected but not pinned.

All three Rust setup uses now pin master and pass a toolchain explicitly.
The original guide step still installs stable first; the conditional MSRV step
then installs the matrix-selected `1.85.0`. The foundations and guides matrices
remain `[stable, "1.85.0"]`, including their existing component selection.

## Compatibility decisions

- All eight JavaScript actions use the `node24` action runtime. The checkout,
  setup-node and setup-python upstream documentation specifies Actions Runner
  `v2.327.1` or later. These workflows use maintained GitHub-hosted runners;
  the action runtime is separate from the selected Python 3.12, Node 22 and
  Bun 1.4.4 application tools, which remain unchanged.
- GitHub's [macOS 14 retirement notice](https://github.com/actions/runner-images/issues/13518)
  says all GitHub Actions macOS 14 images retired on 2026-11-02. The guide job
  now explicitly selects `macos-26`, preserving macOS-only guide execution.
  The [official runner table](https://github.com/actions/runner-images/blob/main/README.md)
  maps that label to macOS 26 Arm64; its
  [maintained image documentation](https://github.com/actions/runner-images/blob/main/images/macos/macos-26-arm64-Readme.md)
  was checked. Linux jobs retain their existing `ubuntu-latest` label.
- Checkout v7's new fork restrictions apply to `pull_request_target` and
  `workflow_run`; neither trigger is used here. No unsafe opt-in is added.
  Its credential storage and ESM changes need no caller changes in these jobs.
- Setup-python v7's ESM conversion preserves its caller API. Setup-node's
  automatic npm caching and removed dummy `NODE_AUTH_TOKEN` fallback do not
  affect these Bun installs: no npm cache or registry-auth inputs are used.
- [Upload-artifact v7](https://github.com/actions/upload-artifact/releases/tag/v7.0.0)
  adds optional direct file uploads; `archive` still defaults to `true`.
  [Download-artifact v8](https://github.com/actions/download-artifact/releases/tag/v8.0.0)
  extracts zipped artifacts directly to the supplied path when selected by
  name, preserving the macOS Rustdoc transfer into the Linux site job.
  Its new default digest-mismatch error is retained.
- Rustdoc and preview uploads retain `include-hidden-files: true` and their
  existing retention days. Upload-pages-artifact v5 retains the single tar
  packaging convention, default artifact name `github-pages` and hidden-file
  exclusion used by [v4.0.0](https://github.com/actions/upload-pages-artifact/blob/v4.0.0/action.yml).
  Its official composite pins nested upload-artifact v7.0.0; this upstream
  implementation is consumed as released.
- Configure-pages still defaults `enablement` to `false`. It only executes
  for Pages artifact preparation. Deploy-pages v5 uses Node 24 and retains
  the required `pages: write`, `id-token: write` and `github-pages` environment.
  `workflow_dispatch`, the default-false `deploy` input, the
  `PAGES_DEPLOY_ENABLED == 'true'` condition and the verify/deploy dependency
  remain unchanged. The reusable workflow reference remains local.
- Artifact/Pages actions are used for GitHub.com. Their upstream GHES caveats
  are not a new supported platform or an authorization to expand hosting.

## Verification

- Latest official [actionlint v1.7.12](https://github.com/rhysd/actionlint/releases/tag/v1.7.12)
  was downloaded for Darwin Arm64 and verified against the release's checksum
  file: archive SHA-256
  `aba9ced2dee8d27fecca3dc7feb1a7f9a52caefa1eb46f3271ea66b6e0e6953f`.
  All three workflows pass:
  `actionlint -shellcheck= -pyflakes= .github/workflows/ci.yml .github/workflows/documentation.yml .github/workflows/pages.yml`.
  ShellCheck and Pyflakes integration was disabled because those executables
  were unavailable; actionlint itself ran and exited zero.
- A local baseline comparison reverted only the audited action-reference
  substitutions, explicit stable input and macOS runner replacement in memory,
  then compared every workflow byte with the starting commit. It passed,
  confirming unchanged validation commands, triggers, permissions, gates,
  matrix toolchains and artifact paths.
- The inventory check found 15 remote action uses, all fully SHA-pinned, and
  one unchanged local reusable-workflow use across exactly three workflows.
- `git diff --check` passed. Actual hosted runs, Rust/docs/site build execution,
  repository settings, commits, pushes and deployment are outside this audit.

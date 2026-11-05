#!/usr/bin/env bash
# Source from the repository root: source .github/scripts/rust-tools.sh stable
# Explicit tools prevent accidental use of a system/Homebrew Rust installation.
set -euo pipefail
cordis_tc="${1:?expected stable or 1.85.0}"
case "$cordis_tc" in stable|1.85.0) ;; *) return 2 ;; esac
cordis_bin=""
for cordis_tool in cargo rustc rustdoc cargo-clippy clippy-driver; do
  cordis_path="$(rustup which --toolchain "$cordis_tc" "$cordis_tool")"
  test -x "$cordis_path"
  case "$cordis_path" in /*) ;; *) return 2 ;; esac
  if test -z "$cordis_bin"; then cordis_bin="$(dirname "$cordis_path")"; fi
  test "$(dirname "$cordis_path")" = "$cordis_bin"
  case "$cordis_tool" in
    cargo) export CORDIS_CARGO="$cordis_path" ;;
    rustc) export RUSTC="$cordis_path" ;;
    rustdoc) export RUSTDOC="$cordis_path" ;;
  esac
done
export RUSTUP_TOOLCHAIN="$cordis_tc"
export PATH="$cordis_bin:$PATH"
export RUSTDOCFLAGS="-D warnings"
if test "$cordis_tc" = stable; then
  export RUSTFMT="$(rustup which --toolchain stable rustfmt)"
  cordis_fmt="$(rustup which --toolchain stable cargo-fmt)"
  test "$(dirname "$RUSTFMT")" = "$cordis_bin"
  test "$(dirname "$cordis_fmt")" = "$cordis_bin"
  "$RUSTFMT" --version
fi
for cordis_tool in cargo rustc rustdoc cargo-clippy clippy-driver; do
  "$cordis_bin/$cordis_tool" --version
done

# MCN App Catalog — candidate foundation

An informational app catalog and offline Rust validator, separate from the host application. **This is not an app store or installer.** There are no signing keys, release packages, downloader, installer, app host or host integration. Original catalog code, schema, documentation and curated metadata are licensed **MIT OR Apache-2.0**, at your option; see [LICENSING.md](LICENSING.md).

## What works

- A real informational catalog, `catalog.json`, with WordCraft as the first candidate.
- An offline Rust validator and `check` / `list` commands.
- A Draft 2020-12 structural schema in `schemas/`; the Rust validator adds semantic checks and is the acceptance gate for this draft.
- Size, count, field/type, duplicate-key, unique-ID, display-control, exact commit and source-bound evidence checks. No network or execution in the validator.

Run from this directory with Rust 1.98.1:

```sh
cargo test --locked --offline
cargo clippy --locked --offline --all-targets -- -D warnings
cargo run --locked --offline -- check catalog.json
cargo run --locked --offline -- list catalog.json
```

Offline commands require the pinned toolchain and Cargo dependencies to be installed/cached first. On a clean machine, run `cargo fetch --locked` online before the offline checks. See [TESTING.md](TESTING.md) for the tested scope.

`check` returns success for a valid **blocked candidate**, not for installation eligibility. Every candidate must have `installable: false` and all five unresolved gates. Unknown fields (including download URLs, commands and permission grants) are rejected. Errors never echo caller-controlled data. CLI reads a caller-selected regular file with a 256 KiB byte limit and writes only its result to stdout/stderr. It is not an archive reader or a sandbox.

Limits: 256 KiB input, 1–256 unique candidates, 100-byte IDs, 80-byte display names and 128-byte opaque source-version labels. IDs and source tokens are never repaired/normalized. Structural schema string counts are supplemented by these stricter byte/display checks in Rust. License evidence must match the candidate's exact repository, commit and root Cargo.toml. This first profile deliberately recognizes only `MIT OR Apache-2.0`; it is not a general SPDX parser or a grant for an app's assets. Schema evaluation alone does not detect all semantic errors or authenticate provenance.

## WordCraft evidence and limitations

The public commit `aadb077e072b7de05f724e359d8db82e151f125c` in `storytold/wordcraft` was verified through GitHub's commit API. At that commit, root Cargo.toml declares source version **0.12.0** and **MIT OR Apache-2.0**. The pinned LICENSE-MIT path was also checked. GitHub's single detected license label is not substituted for the manifest's literal OR expression.

- Source: https://github.com/storytold/wordcraft/commit/aadb077e072b7de05f724e359d8db82e151f125c
- Declaration: https://github.com/storytold/wordcraft/blob/aadb077e072b7de05f724e359d8db82e151f125c/Cargo.toml
- License: https://github.com/storytold/wordcraft/blob/aadb077e072b7de05f724e359d8db82e151f125c/LICENSE-MIT

This is a **source snapshot**, not a bound binary release or an assertion that it is latest. No app archive, images, fonts, executable, patches or upstream implementation code is included. The source declaration does not clear dependencies, assets, fonts, trademarks, wrappers or release redistribution. Offline, OS sandbox, custom-origin runtime and brokered state-adapter compatibility have not been tested. The catalog cannot turn these gates green: a production trust/release format and verified review evidence are still required.

## Publication boundary

This repository contains only new catalog sources, candidate metadata, schema, lockfile, license texts and public documentation. Private host sources, plans, internal research/review reports, execution logs, local user paths, target outputs, credentials and reference trees are excluded. No hosted CI or paid services are configured. Publishing this catalog does not publish or change the private host application. Candidate metadata is not authenticated installation authority; release signing, package reviews and host compatibility remain separate work.

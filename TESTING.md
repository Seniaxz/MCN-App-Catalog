# Testing and scope

Run from the repository root with the pinned Rust toolchain. On a fresh machine fetch dependencies with `cargo fetch --locked` before using `--offline`.

```sh
cargo fmt --check
cargo test --locked --offline
cargo clippy --locked --offline --all-targets -- -D warnings
cargo check --locked --offline --lib --target wasm32-unknown-unknown
cargo run --locked --offline -- check catalog.json
cargo run --locked --offline -- list catalog.json
```

The WASM target must be installed separately with `rustup target add wasm32-unknown-unknown` for the pinned toolchain.

Local Windows verification: six parser/validation test groups and three CLI integration tests passed. Formatting, Clippy with warnings denied, and WASM library compilation passed. `check` accepted the real WordCraft candidate and reported `1 app(s); 0 installable releases`; `list` reported its pinned source and pending gates. The first positive regression was observed failing before the validator was implemented.

Tests cover unknown/duplicate/missing fields, schema kind/version, types, duplicate app IDs, unsafe repository URLs, malformed commits, display controls, source-bound license evidence, missing/duplicate gates, release/install fields, bounded malformed/oversized input, unsupported CLI commands and bad file arguments. JSON Schema was checked using a Draft 2020-12 validator; the real candidate passed and `installable: true` failed. Schema success is not semantic acceptance or authentication.

These are **catalog metadata and CLI tests**, not execution of WordCraft. There is no claim of app-package integrity, dependency/asset license clearance, signed release trust, OS sandbox enforcement, offline app operation, state-adapter compatibility, Linux/macOS runtime validation, independent security audit or hosted CI. No app installer or app-host code is present. All candidates remain non-installable.

// SPDX-FileCopyrightText: MCN App Catalog contributors
// SPDX-License-Identifier: MIT OR Apache-2.0
use std::process::Command;

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mcn-catalog"))
}
const CATALOG: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/catalog.json");

#[test]
fn check_reports_candidate_not_installation_approval() {
    let output = command().args(["check", CATALOG]).output().unwrap();
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("1 app(s); 0 installable releases")
    );
}

#[test]
fn list_reports_real_wordcraft_and_pending_gates() {
    let output = command().args(["list", CATALOG]).output().unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    for expected in [
        "WordCraft",
        "NOT INSTALLABLE",
        "aadb077e072b7de05f724e359d8db82e151f125c",
        "owner-release-approval",
    ] {
        assert!(text.contains(expected));
    }
}

#[test]
fn cli_rejects_install_fetch_missing_files_and_unexpected_arguments() {
    for args in [
        vec![],
        vec!["install", CATALOG],
        vec!["fetch", CATALOG],
        vec!["check"],
        vec!["check", CATALOG, "extra"],
        vec!["check", "missing-catalog-test-file.json"],
        vec!["check", env!("CARGO_MANIFEST_DIR")],
    ] {
        let output = command().args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

// SPDX-FileCopyrightText: MCN App Catalog contributors
// SPDX-License-Identifier: MIT OR Apache-2.0
use mcn_catalog::validate;
use serde_json::{Value, json};

fn candidate() -> Value {
    serde_json::from_str(include_str!("../catalog.json")).unwrap()
}
fn accepted(value: &Value) -> bool {
    validate(&serde_json::to_vec(value).unwrap()).is_ok()
}

#[test]
fn real_wordcraft_candidate_is_valid_but_not_installable() {
    let value = candidate();
    assert!(accepted(&value));
    assert_eq!(value["apps"][0]["installable"], false);
}

#[test]
fn release_fields_and_automatic_approval_are_rejected() {
    for (field, value) in [
        ("installable", json!(true)),
        ("status", json!("ready")),
        ("download_url", json!("https://example.org/app.zip")),
        ("permissions", json!({"mail_data":"read"})),
        ("install_command", json!("program")),
    ] {
        let mut data = candidate();
        data["apps"][0][field] = value;
        assert!(!accepted(&data), "accepted {field}");
    }
}

#[test]
fn schemas_missing_fields_duplicates_and_wrong_types_are_rejected() {
    for field in ["schema_version", "kind", "apps"] {
        let mut data = candidate();
        data.as_object_mut().unwrap().remove(field);
        assert!(!accepted(&data));
    }
    for value in [json!(2), json!(0), json!(-1), json!("1"), json!(null)] {
        let mut data = candidate();
        data["schema_version"] = value;
        assert!(!accepted(&data));
    }
    let mut data = candidate();
    data["extra"] = json!(true);
    assert!(!accepted(&data));
    let mut data = candidate();
    data["kind"] = json!("mcn-releases");
    assert!(!accepted(&data));
    let mut data = candidate();
    data["apps"] = json!([]);
    assert!(!accepted(&data));
    let mut data = candidate();
    let app = data["apps"][0].clone();
    data["apps"].as_array_mut().unwrap().push(app);
    assert!(!accepted(&data));
    let bytes = serde_json::to_string(&candidate()).unwrap().replace(
        "\"schema_version\":1",
        "\"schema_version\":1,\"schema_version\":1",
    );
    assert!(validate(bytes.as_bytes()).is_err());
}

#[test]
fn hostile_urls_source_pins_ids_and_labels_are_rejected() {
    for repository in [
        "http://github.com/storytold/wordcraft",
        "https://127.0.0.1/a/b",
        "https://github.com@127.0.0.1/a/b",
        "https://github.com/storytold/wordcraft?query=value",
        "https://github.com/storytold/wordcraft/extra",
        "https://github.com/../wordcraft",
        "https://github.com/storytold/wordcraft/",
    ] {
        let mut data = candidate();
        data["apps"][0]["repository"] = json!(repository);
        assert!(!accepted(&data));
    }
    for commit in [
        "main",
        "aadb077",
        "aadb077e072b7de05f724e359d8db82e151f125C",
        "../source",
    ] {
        let mut data = candidate();
        data["apps"][0]["source_commit"] = json!(commit);
        assert!(!accepted(&data));
    }
    for name in ["", "Word\nCraft", "Word\u{202e}Craft"] {
        let mut data = candidate();
        data["apps"][0]["name"] = json!(name);
        assert!(!accepted(&data));
    }
    for id in [
        "../wordcraft",
        "io.github.Seniaxz.wordcraft",
        "io..wordcraft",
        "wordcraft",
        "io.github.seniaxz/wordcraft",
    ] {
        let mut data = candidate();
        data["apps"][0]["id"] = json!(id);
        assert!(!accepted(&data));
    }
}

#[test]
fn license_evidence_and_all_blockers_must_remain_bound() {
    for (field, value) in [
        ("scope", json!("release-approved")),
        ("expression", json!("approved")),
        (
            "evidence_url",
            json!("https://github.com/storytold/wordcraft/blob/main/Cargo.toml"),
        ),
        ("extra", json!(true)),
    ] {
        let mut data = candidate();
        data["apps"][0]["license"][field] = value;
        assert!(!accepted(&data));
    }
    for blockers in [
        json!([]),
        json!(["package-review"]),
        json!(["unknown"]),
        json!([
            "package-review",
            "package-review",
            "offline-tests",
            "sandbox-tests",
            "state-adapter",
            "owner-release-approval"
        ]),
    ] {
        let mut data = candidate();
        data["apps"][0]["blockers"] = blockers;
        assert!(!accepted(&data));
    }
}

#[test]
fn hostile_and_oversized_inputs_return_errors_without_unwinding() {
    for input in [
        vec![],
        vec![255; 64],
        vec![b'['; 4096],
        vec![b' '; 262145],
        br#"{"schema_version":1,"kind":"mcn-candidates","apps":null}"#.to_vec(),
    ] {
        let result = std::panic::catch_unwind(|| validate(&input));
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }
    let mut data = candidate();
    let app = data["apps"][0].clone();
    data["apps"] = json!(vec![app; 257]);
    assert!(!accepted(&data));
}

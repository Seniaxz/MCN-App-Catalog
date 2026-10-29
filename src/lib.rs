// SPDX-FileCopyrightText: MCN App Catalog contributors
// SPDX-License-Identifier: MIT OR Apache-2.0
#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use serde::Deserialize;
use std::{collections::BTreeSet, fmt};

pub const MAX_CATALOG_BYTES: usize = 262_144;
pub const MAX_APPS: usize = 256;
const BLOCKERS: [&str; 5] = [
    "package-review",
    "offline-tests",
    "sandbox-tests",
    "state-adapter",
    "owner-release-approval",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub kind: String,
    pub apps: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub status: String,
    pub installable: bool,
    pub repository: String,
    pub source_commit: String,
    pub source_version: String,
    pub license: License,
    pub blockers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub expression: String,
    pub scope: String,
    pub evidence_url: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    TooLarge,
    Json,
    Catalog,
    Candidate,
    Id,
    Source,
    License,
    Blockers,
    Label,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::TooLarge => "Catalog exceeds the 256 KiB limit",
            Self::Json => "Invalid catalog JSON, fields or types",
            Self::Catalog => "Unsupported catalog version/kind or invalid candidate count",
            Self::Candidate => "Candidate catalogs cannot approve installation or releases",
            Self::Id => "Invalid or duplicate candidate ID",
            Self::Source => "Expected a bounded HTTPS GitHub repository and exact lowercase 40-hex source commit",
            Self::License => "Unsupported source license declaration or evidence not bound to the source commit",
            Self::Blockers => "All five unfulfilled candidate gates must be present exactly once",
            Self::Label => "Candidate label/version is empty, too long or contains display-control characters",
        })
    }
}
impl std::error::Error for Error {}

fn label_valid(s: &str, max: usize) -> bool {
    !s.trim().is_empty() && s.len() <= max && !s.chars().any(|c| c.is_control() || matches!(c, '\u{200b}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2060}'..='\u{206f}' | '\u{feff}'))
}

fn id_valid(id: &str) -> bool {
    let count = id.split('.').count();
    id.len() <= 100
        && (3..=6).contains(&count)
        && id.split('.').all(|part| {
            part.as_bytes()
                .first()
                .is_some_and(|b| b.is_ascii_lowercase())
                && !part.ends_with('-')
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        })
}

fn repository_valid(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://github.com/") else {
        return false;
    };
    let Some((owner, repository)) = rest.split_once('/') else {
        return false;
    };
    !owner.is_empty()
        && owner.len() <= 39
        && !owner.starts_with('-')
        && !owner.ends_with('-')
        && owner
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        && !repository.is_empty()
        && repository.len() <= 100
        && repository != "."
        && repository != ".."
        && repository
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}

/// Parse informational candidates only. Does not fetch, authenticate, approve or install.
pub fn parse(bytes: &[u8]) -> Result<Catalog, Error> {
    if bytes.len() > MAX_CATALOG_BYTES {
        return Err(Error::TooLarge);
    }
    // Typed deserialization rejects duplicate/unknown keys; serde_json retains its recursion limit.
    let catalog: Catalog = serde_json::from_slice(bytes).map_err(|_| Error::Json)?;
    if catalog.schema_version != 1
        || catalog.kind != "mcn-candidates"
        || catalog.apps.is_empty()
        || catalog.apps.len() > MAX_APPS
    {
        return Err(Error::Catalog);
    }
    let mut ids = BTreeSet::new();
    for app in &catalog.apps {
        if !id_valid(&app.id) || !ids.insert(app.id.as_str()) {
            return Err(Error::Id);
        }
        if app.installable || app.status != "blocked" {
            return Err(Error::Candidate);
        }
        if !label_valid(&app.name, 80) || !label_valid(&app.source_version, 128) {
            return Err(Error::Label);
        }
        if !repository_valid(&app.repository)
            || app.source_commit.len() != 40
            || !app
                .source_commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::Source);
        }
        // Deliberately narrow initial profile, not a general SPDX parser or a redistribution grant.
        if app.license.expression != "MIT OR Apache-2.0"
            || app.license.scope != "source-snapshot-only"
            || app.license.evidence_url
                != format!("{}/blob/{}/Cargo.toml", app.repository, app.source_commit)
        {
            return Err(Error::License);
        }
        let blockers: BTreeSet<_> = app.blockers.iter().map(String::as_str).collect();
        if app.blockers.len() != BLOCKERS.len()
            || blockers.len() != BLOCKERS.len()
            || !BLOCKERS.iter().all(|b| blockers.contains(b))
        {
            return Err(Error::Blockers);
        }
    }
    Ok(catalog)
}

pub fn validate(bytes: &[u8]) -> Result<(), Error> {
    parse(bytes).map(|_| ())
}

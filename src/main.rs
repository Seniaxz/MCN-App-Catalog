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
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process::ExitCode,
};

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let command = args
        .next()
        .ok_or("Usage: mcn-catalog check|list CATALOG.json")?;
    if command != "check" && command != "list" {
        return Err("Expected check or list".into());
    }
    let path = args.next().ok_or("Expected a catalog file")?;
    if args.next().is_some() {
        return Err("Unexpected arguments".into());
    }
    let file = File::open(path).map_err(|_| "Cannot open catalog file")?;
    if !file
        .metadata()
        .map_err(|_| "Cannot inspect catalog file")?
        .is_file()
    {
        return Err("Expected a regular catalog file".into());
    }
    let mut bytes = Vec::new();
    file.take(mcn_catalog::MAX_CATALOG_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read catalog file")?;
    let catalog = mcn_catalog::parse(&bytes).map_err(|error| error.to_string())?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    if command == "check" {
        writeln!(
            out,
            "Valid candidate catalog: {} app(s); 0 installable releases",
            catalog.apps.len()
        )
        .map_err(|_| "Cannot write output")?;
    } else {
        for app in &catalog.apps {
            writeln!(
                out,
                "{}\t{}\tblocked\tNOT INSTALLABLE\tsource {}",
                app.id, app.name, app.source_version
            )
            .map_err(|_| "Cannot write output")?;
            writeln!(out, "  {} @ {}", app.repository, app.source_commit)
                .map_err(|_| "Cannot write output")?;
            writeln!(out, "  Pending: {}", app.blockers.join(", "))
                .map_err(|_| "Cannot write output")?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "{error}");
            ExitCode::FAILURE
        }
    }
}

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

use anyhow::Error;

use crate::licenses::{License, LicenseId, LicensesInterner};

const PATINA_COPYRIGHT: &str = "The Patina Project Contributors";
const RUST_COPYRIGHT: &str = "The Rust Project Developers (see https://thanks.rust-lang.org)";
const RUST_LICENSES_DIR: &str = "rust/LICENSES/";
const RUST_DIR: &str = "rust/";

pub(crate) fn collect(
    rust_snapshot_commit: &str,
    interner: &mut LicensesInterner,
) -> Result<Vec<(PathBuf, LicenseId)>, Error> {
    let snapshot = RustSnapshot::load(rust_snapshot_commit)?;

    println!("gathering license information from REUSE (this might take a minute...)");
    let start = Instant::now();
    let raw = &obtain_spdx_document()?;
    println!("finished gathering the license information from REUSE in {:.2?}", start.elapsed());

    let files = crate::spdx::parse_tag_value(raw)?;

    let mut result = Vec::new();
    for file in files {
        let path = file.name.trim_start_matches("./");
        if path.starts_with(RUST_LICENSES_DIR) {
            continue;
        }

        let drop = match snapshot.status(path) {
            RustSnapshotFileStatus::Unchanged => Some(PATINA_COPYRIGHT),
            RustSnapshotFileStatus::Changed => None,
            RustSnapshotFileStatus::NotPresent if path.starts_with(RUST_DIR) => {
                Some(RUST_COPYRIGHT)
            }
            RustSnapshotFileStatus::NotPresent => None,
        };
        let license = interner.intern(License {
            spdx: file.concluded_license,
            copyright: file
                .copyright_text
                .split('\n')
                .filter(|s| !drop.is_some_and(|drop| s.trim_end().ends_with(drop)))
                .map(|s| s.into())
                .collect(),
        });
        result.push((file.name.into(), license));
    }

    Ok(result)
}

#[derive(Clone, Copy)]
enum RustSnapshotFileStatus {
    /// The file is in the snapshot, and its contents haven't changed since.
    /// Renaming doesn't count as changing.
    Unchanged,
    /// The file is in the snapshot, or was renamed from a file in it, and has changed since.
    Changed,
    /// The file is not in the snapshot.
    NotPresent,
}

struct RustSnapshot {
    files: HashMap<String, RustSnapshotFileStatus>,
}

impl RustSnapshot {
    fn load(commit: &str) -> Result<Self, Error> {
        let tree = git(&["ls-tree", "-r", "-z", "--name-only", commit])?;

        let mut files: HashMap<String, RustSnapshotFileStatus> = tree
            .split_terminator('\0')
            .map(|path| (path.into(), RustSnapshotFileStatus::Unchanged))
            .collect();

        let diff = git(&["diff", "-z", "--name-status", "-M", "-l0", commit])?;
        let fields: Vec<&str> = diff.split_terminator('\0').collect();
        let mut rest = &fields[..];
        while !rest.is_empty() {
            rest = match rest {
                [status, old, new, rest @ ..] if status.starts_with('R') => {
                    files.remove(*old);
                    let new_status = if *status == "R100" {
                        RustSnapshotFileStatus::Unchanged
                    } else {
                        RustSnapshotFileStatus::Changed
                    };
                    files.insert(new.to_string(), new_status);
                    rest
                }
                ["A", _, rest @ ..] => rest,
                ["D", path, rest @ ..] => {
                    files.remove(*path);
                    rest
                }
                [_, path, rest @ ..] => {
                    files.insert(path.to_string(), RustSnapshotFileStatus::Changed);
                    rest
                }
                _ => anyhow::bail!("malformed `git diff` output"),
            };
        }

        Ok(Self { files })
    }

    fn status(&self, path: &str) -> RustSnapshotFileStatus {
        self.files.get(path).copied().unwrap_or(RustSnapshotFileStatus::NotPresent)
    }
}

fn git(args: &[&str]) -> Result<String, Error> {
    let output = Command::new("git").args(args).stderr(Stdio::inherit()).output()?;
    if !output.status.success() {
        anyhow::bail!("`git {}` failed", args.join(" "));
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn obtain_spdx_document() -> Result<String, Error> {
    let output = Command::new("uvx")
        .args(&[
            "reuse",
            "--include-submodules",
            "spdx",
            "--add-license-concluded",
            "--creator-person=bors",
        ])
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        eprintln!();
        eprintln!("Note that Rust requires some REUSE features that might not be present in the");
        eprintln!("release you're using. Make sure your REUSE release includes these PRs:");
        eprintln!();
        eprintln!(" - https://github.com/fsfe/reuse-tool/pull/623");
        eprintln!();
        anyhow::bail!("collecting licensing information with REUSE failed");
    }

    Ok(String::from_utf8(output.stdout)?)
}

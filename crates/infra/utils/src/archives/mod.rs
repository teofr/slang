use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use tar::{Archive, EntryType};

use crate::paths::PathExtensions;

/// Options controlling how [`extract_tarball`] unpacks an archive.
#[derive(Clone, Copy, Default)]
pub struct ExtractOptions<'a> {
    /// Gz-decode the reader before reading tar entries (i.e. the input is a
    /// `.tar.gz` rather than a plain `.tar`).
    pub gzip: bool,
    /// When set, only entries whose path contains this segment are extracted,
    /// and everything up to and including it is stripped, so `dest` ends up
    /// holding the subtree's contents directly (e.g. `subtree =
    /// "test/libsolidity/semanticTests"` extracts that directory's contents
    /// into `dest`). When `None`, the whole archive is unpacked into `dest`.
    pub subtree: Option<&'a str>,
}

/// The outcome of [`extract_tarball`].
pub struct Extracted {
    /// The commit SHA read from the archive's `pax_global_header` comment, if
    /// present. GitHub's codeload tarballs carry the resolved commit here, which
    /// lets callers pin the exact content a (mutable) tag pointed at. Only
    /// populated on the subtree path; `None` for a whole-archive unpack.
    pub commit_sha: Option<String>,
    /// Number of files written (only meaningful on the subtree path; `0` for a
    /// whole-archive unpack, which defers counting to `tar`).
    pub file_count: usize,
}

/// Downloads-adjacent helper that unpacks a (already-fetched) tar stream into
/// `dest`, optionally gz-decoding it first and/or extracting only a subtree.
///
/// Both the sourcify runner (whole-archive, plain tar) and the solc-comparison
/// runner (gz-decoded, `semanticTests` subtree, commit-pinned) go through here
/// so the tar handling lives in one place.
pub fn extract_tarball(
    reader: impl Read,
    dest: &Path,
    options: ExtractOptions<'_>,
) -> Result<Extracted> {
    if options.gzip {
        extract(GzDecoder::new(reader), dest, options.subtree)
    } else {
        extract(reader, dest, options.subtree)
    }
}

fn extract(reader: impl Read, dest: &Path, subtree: Option<&str>) -> Result<Extracted> {
    let mut archive = Archive::new(reader);

    // Whole-archive unpack: defer to tar's own `unpack`, which handles the full
    // directory structure exactly as callers relied on before this helper.
    let Some(subtree) = subtree else {
        archive
            .unpack(dest)
            .with_context(|| format!("Failed to unpack archive into {dest:?}"))?;
        return Ok(Extracted {
            commit_sha: None,
            file_count: 0,
        });
    };

    // Subtree extraction: walk entries, capture the commit SHA from the pax
    // global header, and unpack only files under `subtree` (rebased so `dest`
    // holds the subtree contents directly).
    let mut commit_sha = None;
    let mut file_count = 0;
    for entry in archive.entries()? {
        let mut entry = entry?;

        if entry.header().entry_type() == EntryType::XGlobalHeader {
            let mut header = String::new();
            entry.read_to_string(&mut header).ok();
            if let Some(sha) = parse_pax_comment(&header) {
                commit_sha = Some(sha);
            }
            continue;
        }

        let entry_path = entry.path()?.into_owned();
        let Some(relative) = strip_to_subtree(&entry_path, subtree) else {
            continue;
        };
        if entry.header().entry_type().is_dir() {
            continue;
        }

        let target = dest.join(relative);
        fs::create_dir_all(target.unwrap_parent())?;
        entry
            .unpack(&target)
            .with_context(|| format!("Failed to unpack entry into {target:?}"))?;
        file_count += 1;
    }

    Ok(Extracted {
        commit_sha,
        file_count,
    })
}

/// Extracts the `comment=<sha>` value from a pax header's records (each record
/// is `"<len> key=value\n"`).
fn parse_pax_comment(header: &str) -> Option<String> {
    let start = header.find("comment=")? + "comment=".len();
    let value = header[start..].lines().next()?.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

/// Returns the portion of `path` after the `<subtree>/` segment, if present.
/// GitHub archives wrap everything in a single top-level directory (e.g.
/// `solidity-0.8.35/`), so the subtree is matched anywhere in the path.
fn strip_to_subtree(path: &Path, subtree: &str) -> Option<PathBuf> {
    let path_str = path.to_str()?;
    let needle = format!("{subtree}/");
    let index = path_str.find(&needle)?;
    let relative = &path_str[index + needle.len()..];
    (!relative.is_empty()).then(|| PathBuf::from(relative))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pax_commit_comment() {
        let header = "52 comment=1234567890abcdef1234567890abcdef12345678\n";
        assert_eq!(
            parse_pax_comment(header).as_deref(),
            Some("1234567890abcdef1234567890abcdef12345678")
        );
        assert_eq!(parse_pax_comment("30 mtime=1700000000.0\n"), None);
        assert_eq!(parse_pax_comment("14 comment=\n"), None);
    }

    #[test]
    fn strips_to_subtree_segment() {
        assert_eq!(
            strip_to_subtree(
                Path::new("solidity-0.8.35/test/libsolidity/semanticTests/various/erc20.sol"),
                "test/libsolidity/semanticTests"
            ),
            Some(PathBuf::from("various/erc20.sol"))
        );
        // The subtree directory entry itself (nothing after it) is skipped.
        assert_eq!(
            strip_to_subtree(
                Path::new("solidity-0.8.35/test/libsolidity/semanticTests/"),
                "test/libsolidity/semanticTests"
            ),
            None
        );
        // A path that doesn't contain the subtree is skipped.
        assert_eq!(
            strip_to_subtree(Path::new("solidity-0.8.35/README.md"), "semanticTests"),
            None
        );
    }
}

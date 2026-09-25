//! `.envo-config`: the per-project settings `push` and `pull` remember, so a
//! tag and its owner only have to be typed once — the way `git pull` works
//! after the first `git remote add`.
//!
//! It lives next to `.env` in the directory the command runs in, as JSON:
//!
//! ```json
//! {
//!   "tag": "my-project",
//!   "owners": { "my-project": "npub1..." }
//! }
//! ```
//!
//! `tag` is the default for `envo push` / `envo pull` run without one.
//! `owners` is the per-tag owner pin: which pubkey is allowed to publish that
//! tag. A tag is just a `d` tag on a public relay, so anyone can publish under
//! it and address a recipients entry to us; the author is the one thing `pull`
//! cannot infer, so a human decides it once with `--owner` and it is pinned
//! here. That makes the file a trust anchor — whoever can rewrite it can
//! redirect a tag to their own identity — so it is written owner-only and
//! added to the project's `.gitignore` before it is ever written.

use crate::helper::log;
use crate::helper::secret_file::write_secret;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const CONFIG_FILE: &str = ".envo-config";

#[derive(Debug, Default, Serialize, Deserialize)]
struct ProjectConfig {
    /// The tag used when none is given on the command line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    /// `{tag: npub}`, the only publisher `pull` accepts for each tag.
    #[serde(default)]
    owners: HashMap<String, String>,
}

/// Returns the tag to use: the one given on the command line, otherwise the
/// one saved in `.envo-config`. Nothing is written here; callers remember a
/// tag with [`remember_tag`] once it has actually worked.
pub fn resolve_tag(tag: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(tag) = tag {
        return Ok(tag);
    }

    read_config()?.tag.ok_or_else(|| {
        format!(
            "No tag given and none saved in {}. Pass one once, e.g. `envo push <tag>`.",
            CONFIG_FILE
        )
        .into()
    })
}

/// Saves `tag` as the default for later runs in this directory.
pub fn remember_tag(tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = read_config().unwrap_or_default();
    config.tag = Some(tag.to_string());
    write_config(&config)
}

/// Returns the pubkey pinned for `tag`, if the user has trusted one.
///
/// A damaged file reads as "nothing is pinned" rather than an error: the only
/// thing a caller does with an absent pin is refuse to pull and ask for
/// `--owner`, so falling back here cannot loosen a trust decision, it can only
/// ask for it again. This is the one place that reports the damage, so the
/// user does not hear about it twice in a run that also saves a pin.
pub fn get_trusted_owner(tag: &str) -> Option<String> {
    match read_config() {
        Ok(config) => config.owners.get(tag).cloned(),
        Err(e) => {
            log::warn(&format!("Could not read the trusted owner list: {}", e));
            None
        }
    }
}

/// Pins `owner_pubkey` as the only publisher `pull` will accept for `tag`.
///
/// The rest of the file is read back and rewritten so pinning one tag does not
/// drop the others or the default tag. A file too damaged to read starts over
/// from empty, which loses the other pins but never silently keeps a stale
/// one: `get_trusted_owner` has already warned about the damage by this point.
pub fn save_trusted_owner(tag: &str, owner_pubkey: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = read_config().unwrap_or_default();
    config
        .owners
        .insert(tag.to_string(), owner_pubkey.to_string());
    write_config(&config)
}

/// Returns the path to `.envo-config` in the current directory.
fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("could not read the current directory: {}", e))?;

    Ok(current_dir.join(CONFIG_FILE))
}

/// Reads the whole config. A missing or empty file is the default config, not
/// an error; only a file that is there but unreadable as JSON fails.
fn read_config() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
    let path = config_path()?;

    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Ok(ProjectConfig::default());
    };

    if contents.trim().is_empty() {
        return Ok(ProjectConfig::default());
    }

    let config = serde_json::from_str(&contents)
        .map_err(|e| format!("could not parse {}: {}", path.display(), e))?;

    Ok(config)
}

/// Writes the whole config back, owner-only because the pins in it decide
/// whose events `pull` will trust.
///
/// The file is git-ignored first, and nothing is written if that fails: a
/// committed pin lets anyone who can push to the repo point a tag at their own
/// key, so the config should never exist in a state where `git add .` picks it
/// up.
fn write_config(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let contents = serde_json::to_string_pretty(config)
        .map_err(|e| format!("could not encode {}: {}", CONFIG_FILE, e))?;

    let path = config_path()?;

    if let Some(dir) = path.parent() {
        ensure_gitignored(dir)?;
    }

    write_secret(&path, &contents)
}

/// Adds `.envo-config` to `dir/.gitignore` when `dir` is inside a git
/// repository and the entry is not there yet.
///
/// Outside a repository there is nothing to protect against, so no
/// `.gitignore` is created. Only an exact `.envo-config` or `/.envo-config`
/// line counts as present; wider patterns are not evaluated, and at worst the
/// entry is added redundantly.
fn ensure_gitignored(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dir
        .ancestors()
        .any(|ancestor| ancestor.join(".git").exists())
    {
        return Ok(());
    }

    let gitignore = dir.join(".gitignore");
    let existing = std::fs::read_to_string(&gitignore).unwrap_or_default();

    let already_ignored = existing.lines().any(|line| {
        let line = line.trim();
        line == CONFIG_FILE || line.strip_prefix('/') == Some(CONFIG_FILE)
    });

    if already_ignored {
        return Ok(());
    }

    let mut updated = existing;
    if !updated.is_empty() && !updated.ends_with('\n') {
        updated.push('\n');
    }
    updated.push_str(CONFIG_FILE);
    updated.push('\n');

    std::fs::write(&gitignore, updated)
        .map_err(|e| format!("could not update {}: {}", gitignore.display(), e))?;

    Ok(())
}

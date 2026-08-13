//! The per-tag owner pin: which pubkey is allowed to publish a given tag.
//!
//! A tag is just a `d` tag on a public relay, so anyone can publish an event
//! carrying it and address a recipients entry to us. Nothing about the event
//! itself says whether its author is the project owner or someone who guessed
//! the tag name, which makes the author the one thing `pull` cannot infer.
//!
//! So it is decided once, by a human, and remembered: the first `pull` for a
//! tag takes `--owner <npub>` and pins it here, and every later `pull` is
//! constrained to that pubkey. `~/.envo/trusted_owners.json` is a flat
//! `{tag: npub}` map, written owner-only because it is a trust anchor — an
//! attacker who could rewrite it could redirect a tag to their own identity.

use crate::helper::key_valid::envo_dir;
use crate::helper::log;
use crate::helper::secret_file::write_secret;
use std::collections::HashMap;
use std::path::PathBuf;

/// Returns the path to `~/.envo/trusted_owners.json`.
fn pin_file() -> PathBuf {
    envo_dir().join("trusted_owners.json")
}

/// Reads the whole pin map. A missing or empty file is an empty map, not an
/// error; only a file that is there but unreadable as JSON fails.
fn read_pins() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let path = pin_file();

    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Ok(HashMap::new());
    };

    if contents.trim().is_empty() {
        return Ok(HashMap::new());
    }

    let pins = serde_json::from_str(&contents)
        .map_err(|e| format!("could not parse {}: {}", path.display(), e))?;

    Ok(pins)
}

/// Returns the pubkey pinned for `tag`, if the user has trusted one.
///
/// A damaged file reads as "nothing is pinned" rather than an error: the only
/// thing a caller does with an absent pin is refuse to pull and ask for
/// `--owner`, so falling back here cannot loosen a trust decision, it can only
/// ask for it again. This is the one place that reports the damage, so the
/// user does not hear about it twice in a run that also saves a pin.
pub fn get_trusted_owner(tag: &str) -> Option<String> {
    match read_pins() {
        Ok(pins) => pins.get(tag).cloned(),
        Err(e) => {
            log::warn(&format!("Could not read the trusted owner list: {}", e));
            None
        }
    }
}

/// Pins `owner_pubkey` as the only publisher `pull` will accept for `tag`.
///
/// The rest of the map is read back and rewritten so pinning one tag does not
/// drop the others. A file too damaged to read starts over from empty, which
/// loses other tags' pins but never silently keeps a stale one: `get` has
/// already warned about the damage by this point.
pub fn save_trusted_owner(tag: &str, owner_pubkey: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut pins = read_pins().unwrap_or_default();
    pins.insert(tag.to_string(), owner_pubkey.to_string());

    let contents = serde_json::to_string_pretty(&pins)
        .map_err(|e| format!("could not encode the trusted owner list: {}", e))?;

    write_secret(&pin_file(), &contents)?;

    Ok(())
}

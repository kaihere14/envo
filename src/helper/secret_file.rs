//! Filesystem permissions for the files that hold secrets.
//!
//! The whole security model is "possession of the keypair", so the keypair
//! must not be readable by other accounts on the machine. On Unix that means
//! 0600 for files and 0700 for the directory holding them; on other
//! platforms these calls are no-ops and the file inherits the profile's
//! access control.

use std::io::Write;
use std::path::Path;

/// Readable and writable by the owner only.
pub const FILE_MODE: u32 = 0o600;

/// Enterable by the owner only.
pub const DIR_MODE: u32 = 0o700;

/// Tightens an existing file or directory to `mode`.
///
/// Worth calling even on paths that were created with the right mode: it is
/// what repairs identities written by an older version of envo, which left
/// `keys.json` world-readable.
#[cfg(unix)]
pub fn restrict(path: &Path, mode: u32) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::PermissionsExt;

    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).map_err(|e| {
        format!(
            "could not restrict permissions on {}: {}",
            path.display(),
            e
        )
    })?;

    Ok(())
}

#[cfg(not(unix))]
pub fn restrict(_path: &Path, _mode: u32) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

/// Writes `contents` to `path` with owner-only permissions.
pub fn write_secret(path: &Path, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);

    // Creating the file already restricted leaves no window in which the
    // secret exists at the default 0644.
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(FILE_MODE);
    }

    let mut file = options
        .open(path)
        .map_err(|e| format!("could not open {}: {}", path.display(), e))?;

    file.write_all(contents.as_bytes())
        .map_err(|e| format!("could not write {}: {}", path.display(), e))?;

    // `mode` above only applies when the file is created. A file that was
    // already there keeps the permissions it had, so tighten it explicitly.
    restrict(path, FILE_MODE)?;

    Ok(())
}

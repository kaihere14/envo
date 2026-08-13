use std::path::Path;

/// The two project files `push` works from: the secrets to share and the
/// people allowed to read them.
pub struct ProjectFiles {
    pub env_contents: Vec<String>,
    pub trusted_pubkeys: Vec<String>,
}

/// Reads `.env` and `.env-share` from the directory the command was run in.
/// Both are required, so any missing or unreadable file is reported back to
/// the caller instead of being papered over with an empty default.
pub fn load_project_files() -> Result<ProjectFiles, Box<dyn std::error::Error>> {
    // Without a working directory we cannot know which .env to read, so
    // there is nothing sensible to fall back to here.
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("could not read the current directory: {}", e))?;

    let env_contents = parse_env(&read_file(&current_dir.join(".env"))?);
    let trusted_pubkeys = parse_pubkeys(&read_file(&current_dir.join(".env-share"))?);

    Ok(ProjectFiles {
        env_contents,
        trusted_pubkeys,
    })
}

/// Reads a file, tagging the failure with the path so the caller can print a
/// message that says which file was missing.
fn read_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("could not read {}: {}", path.display(), e))?;

    Ok(contents)
}
fn parse_env(contents: &str) -> Vec<String> {
    contents
        .lines() // splits on \n or \r\n, no manual handling needed
        .map(|line| line.trim().trim_end_matches(',')) // strip whitespace AND a trailing comma if present
        .filter(|line| !line.is_empty()) // drop blank lines
        .map(|line| line.to_string())
        .collect()
}

/// Splits the trusted-key list into individual keys.
/// Entries are comma separated; surrounding whitespace and empty entries
/// (a trailing comma, a blank line) are dropped.
fn parse_pubkeys(contents: &str) -> Vec<String> {
    contents
        .split(',')
        .map(|key| key.trim().to_string())
        .filter(|key| !key.is_empty())
        .collect()
}

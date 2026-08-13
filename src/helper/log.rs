//! Terminal status output, in the style of the GitHub CLI: one symbol, one
//! space, one short line. Progress and results go to stdout so they can be
//! read as a transcript of the command; anything the user has to act on goes
//! to stderr so it survives a redirect.

/// Work that is starting or in flight.
pub fn step(message: &str) {
    println!("- {}", message);
}

/// A step finished and produced a result worth confirming.
pub fn success(message: &str) {
    println!("✓ {}", message);
}

/// Something was skipped or degraded, but the command carries on.
pub fn warn(message: &str) {
    eprintln!("! {}", message);
}

/// The command cannot finish what it was asked to do.
pub fn fail(message: &str) {
    eprintln!("X {}", message);
}

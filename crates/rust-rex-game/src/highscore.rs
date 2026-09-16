//! Minimal best-distance persistence. Deliberately just a single float
//! written to a plain text file next to where the game runs from — no
//! serialization crate needed for one number.

use std::fs;

const HIGHSCORE_FILE: &str = "rust_rex_highscore.txt";

pub fn load() -> f32 {
    fs::read_to_string(HIGHSCORE_FILE)
        .ok()
        .and_then(|contents| contents.trim().parse::<f32>().ok())
        .unwrap_or(0.0)
}

pub fn save(best_km: f32) {
    // Best effort: a failure to persist (e.g. read-only filesystem)
    // should never crash the game.
    let _ = fs::write(HIGHSCORE_FILE, format!("{best_km}"));
}

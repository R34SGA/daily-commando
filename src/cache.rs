// SPDX-License-Identifier: EUPL-1.2
//! Local roster cache — remembers soldier names between runs.
//!
//! Stored as JSON under the user cache directory:
//!   - `$XDG_CACHE_HOME/daily-commando/roster.json`
//!   - or `$HOME/.cache/daily-commando/roster.json`
//!
//! Only names are persisted. The exclusion zone (absentees of one instance)
//! is deliberately *not* cached: absentees are re-marked at each briefing.

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RosterCache {
    #[serde(default)]
    pub names: Vec<String>,
}

/// Filesystem location of the roster cache, `None` when no home is known.
pub fn cache_path() -> Option<PathBuf> {
    let dir = std::env::var_os("XDG_CACHE_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .filter(|v| !v.is_empty())
                .map(|h| PathBuf::from(h).join(".cache"))
        });
    dir.map(|d| d.join("daily-commando").join("roster.json"))
}

/// Load the cached roster. A missing or corrupt file yields an empty cache.
pub fn load() -> RosterCache {
    let Some(path) = cache_path() else { return RosterCache::default() };
    match fs::read_to_string(&path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => RosterCache::default(),
    }
}

/// Persist the full roster (absentees included) for the next run. Best-effort:
/// I/O errors are reported to the caller and never abort the daily.
pub fn save(names: &[String]) -> std::io::Result<()> {
    let Some(path) = cache_path() else { return Ok(()) };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&RosterCache { names: names.to_vec() })
        .map_err(|e| std::io::Error::other(e))?;
    fs::write(&path, json)
}

/// Names of the soldiers actually present, i.e. not in the exclusion zone.
pub fn effective_roster(names: &[String], absent: &[bool]) -> Vec<String> {
    names
        .iter()
        .enumerate()
        .filter(|(i, _)| !absent.get(*i).copied().unwrap_or(false))
        .map(|(_, n)| n.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_roster_drops_absent() {
        let names = vec!["Alice".into(), "Bob".into(), "Carol".into()];
        let absent = vec![false, true, false];
        assert_eq!(
            effective_roster(&names, &absent),
            vec!["Alice".to_string(), "Carol".to_string()]
        );
    }

    #[test]
    fn effective_roster_all_absent_is_empty() {
        let names = vec!["Alice".into(), "Bob".into()];
        let absent = vec![true, true];
        assert!(effective_roster(&names, &absent).is_empty());
    }

    #[test]
    fn effective_roster_accepts_shorter_absent_vec() {
        let names = vec!["Alice".into(), "Bob".into()];
        assert_eq!(effective_roster(&names, &[]), names);
    }

    #[test]
    fn cache_json_round_trip() {
        let cache = RosterCache { names: vec!["Alice".into(), "Bob".into()] };
        let json = serde_json::to_string(&cache).unwrap();
        let back: RosterCache = serde_json::from_str(&json).unwrap();
        assert_eq!(back.names, cache.names);
    }

    #[test]
    fn cache_json_tolerates_unknown_fields() {
        let json = r#"{"names":["Alice"],"future_field":42}"#;
        let back: RosterCache = serde_json::from_str(json).unwrap();
        assert_eq!(back.names, vec!["Alice".to_string()]);
    }

    #[test]
    fn cache_json_defaults_when_names_missing() {
        let back: RosterCache = serde_json::from_str("{}").unwrap();
        assert!(back.names.is_empty());
    }
}

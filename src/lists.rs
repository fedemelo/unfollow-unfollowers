use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::export::is_deleted_account;

pub fn load_username_list(path: &Path) -> HashSet<String> {
    let Ok(contents) = fs::read_to_string(path) else {
        return HashSet::new();
    };

    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect()
}

/// Writes `usernames` to `path`, one per line and sorted case-insensitively, creating any
/// missing parent directories. Used to persist a snapshot for the next run to diff against.
pub fn save_username_list(path: &Path, usernames: &HashSet<String>) {
    let mut usernames: Vec<&String> = usernames.iter().collect();
    usernames.sort_by_key(|username| username.to_lowercase());

    let mut contents = usernames
        .iter()
        .map(|username| username.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    if !contents.is_empty() {
        contents.push('\n');
    }

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).expect("failed to create snapshot directory");
    }
    fs::write(path, contents).expect("failed to write snapshot file");
}

pub fn apply_manual_filters(
    usernames: &HashSet<String>,
    exclusions: &HashSet<String>,
    known_disabled: &HashSet<String>,
) -> HashSet<String> {
    usernames
        .iter()
        .filter(|username| {
            !is_deleted_account(username)
                && !exclusions.contains(*username)
                && !known_disabled.contains(*username)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn set(items: &[&str]) -> HashSet<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn load_username_list_returns_empty_set_when_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.txt");
        assert_eq!(load_username_list(&missing), HashSet::new());
    }

    #[test]
    fn load_username_list_skips_blank_lines_and_comments() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("list.txt");
        fs::write(&path, "# a comment\n\nalice\n  bob  \n# another\n").unwrap();
        assert_eq!(load_username_list(&path), set(&["alice", "bob"]));
    }

    #[test]
    fn apply_manual_filters_removes_deleted_accounts() {
        let usernames = set(&["alice", "__deleted__abc123"]);
        let result = apply_manual_filters(&usernames, &HashSet::new(), &HashSet::new());
        assert_eq!(result, set(&["alice"]));
    }

    #[test]
    fn apply_manual_filters_removes_exclusions_and_known_disabled() {
        let usernames = set(&["alice", "bob", "carol"]);
        let exclusions = set(&["bob"]);
        let known_disabled = set(&["carol"]);
        let result = apply_manual_filters(&usernames, &exclusions, &known_disabled);
        assert_eq!(result, set(&["alice"]));
    }

    #[test]
    fn save_username_list_writes_sorted_lines_and_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("snapshot.txt");
        save_username_list(&path, &set(&["Bob", "alice"]));
        assert_eq!(fs::read_to_string(&path).unwrap(), "alice\nBob\n");
        assert_eq!(load_username_list(&path), set(&["alice", "Bob"]));
    }

    #[test]
    fn save_username_list_writes_empty_file_for_empty_set() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("snapshot.txt");
        save_username_list(&path, &HashSet::new());
        assert_eq!(fs::read_to_string(&path).unwrap(), "");
    }

    #[test]
    fn save_username_list_creates_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("snapshot.txt");
        save_username_list(&path, &set(&["alice"]));
        assert_eq!(fs::read_to_string(&path).unwrap(), "alice\n");
    }
}

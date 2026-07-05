use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::export::{entry_usernames, label_value_username};

fn write_json(path: &Path, data: &Value) {
    let text = serde_json::to_string_pretty(data).expect("failed to serialize JSON");
    fs::write(path, format!("{text}\n")).expect("failed to write file");
}

pub fn remove_from_following(export_dir: &Path, username: &str) -> bool {
    let path = export_dir.join("following.json");
    if !path.exists() {
        return false;
    }

    let contents = fs::read_to_string(&path).expect("failed to read following.json");
    let mut data: Value = serde_json::from_str(&contents).expect("invalid following.json");

    let entries = data
        .get("relationships_following")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let filtered: Vec<Value> = entries
        .iter()
        .filter(|entry| !entry_usernames(entry).contains(username))
        .cloned()
        .collect();

    if filtered.len() == entries.len() {
        return false;
    }

    data["relationships_following"] = Value::Array(filtered);
    write_json(&path, &data);
    true
}

pub fn remove_from_pending_requests(export_dir: &Path, username: &str) -> bool {
    let path = export_dir.join("pending_follow_requests.json");
    if !path.exists() {
        return false;
    }

    let contents = fs::read_to_string(&path).expect("failed to read pending_follow_requests.json");
    let entries: Vec<Value> =
        serde_json::from_str(&contents).expect("invalid pending_follow_requests.json");

    let filtered: Vec<Value> = entries
        .iter()
        .filter(|entry| label_value_username(entry).as_deref() != Some(username))
        .cloned()
        .collect();

    if filtered.len() == entries.len() {
        return false;
    }

    write_json(&path, &Value::Array(filtered));
    true
}

pub fn remove_from_username_list(path: &Path, username: &str) -> bool {
    if !path.exists() {
        return false;
    }

    let contents = fs::read_to_string(path).expect("failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let filtered: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|line| line.trim() != username)
        .collect();

    if filtered.len() == lines.len() {
        return false;
    }

    fs::write(path, format!("{}\n", filtered.join("\n"))).expect("failed to write file");
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        write_following, write_pending_follow_requests, write_username_list,
    };

    #[test]
    fn remove_from_following_removes_matching_entry() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["alice", "bob"]);

        assert!(remove_from_following(dir.path(), "alice"));

        let contents = fs::read_to_string(dir.path().join("following.json")).unwrap();
        let data: Value = serde_json::from_str(&contents).unwrap();
        let titles: Vec<&str> = data["relationships_following"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["title"].as_str().unwrap())
            .collect();
        assert_eq!(titles, vec!["bob"]);
    }

    #[test]
    fn remove_from_following_returns_false_when_username_absent() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["bob"]);
        assert!(!remove_from_following(dir.path(), "alice"));
    }

    #[test]
    fn remove_from_following_returns_false_when_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!remove_from_following(dir.path(), "alice"));
    }

    #[test]
    fn remove_from_pending_requests_removes_matching_entry() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "erin"]);

        assert!(remove_from_pending_requests(dir.path(), "dave"));

        let contents = fs::read_to_string(dir.path().join("pending_follow_requests.json")).unwrap();
        let entries: Vec<Value> = serde_json::from_str(&contents).unwrap();
        let remaining: Vec<String> = entries.iter().filter_map(label_value_username).collect();
        assert_eq!(remaining, vec!["erin"]);
    }

    #[test]
    fn remove_from_pending_requests_returns_false_when_username_absent() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["erin"]);
        assert!(!remove_from_pending_requests(dir.path(), "dave"));
    }

    #[test]
    fn remove_from_username_list_removes_matching_line() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("list.txt");
        write_username_list(&path, &["# comment", "alice", "bob"]);

        assert!(remove_from_username_list(&path, "alice"));
        let lines: Vec<String> = fs::read_to_string(&path)
            .unwrap()
            .lines()
            .map(str::to_string)
            .collect();
        assert_eq!(lines, vec!["# comment", "bob"]);
    }

    #[test]
    fn remove_from_username_list_returns_false_when_username_absent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("list.txt");
        write_username_list(&path, &["alice"]);
        assert!(!remove_from_username_list(&path, "bob"));
    }

    #[test]
    fn remove_from_username_list_returns_false_when_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing.txt");
        assert!(!remove_from_username_list(&path, "alice"));
    }
}

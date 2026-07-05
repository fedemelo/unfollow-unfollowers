use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

pub fn entry_usernames(entry: &Value) -> HashSet<String> {
    let empty = Vec::new();
    let items = entry
        .get("string_list_data")
        .and_then(Value::as_array)
        .unwrap_or(&empty);

    items
        .iter()
        .map(|item| {
            item.get("value")
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| {
                    let href = item.get("href").and_then(Value::as_str).unwrap_or("");
                    href.trim_end_matches('/')
                        .rsplit('/')
                        .next()
                        .unwrap_or("")
                        .to_string()
                })
        })
        .collect()
}

fn usernames_from_string_list_data(entries: &[Value]) -> HashSet<String> {
    entries.iter().flat_map(entry_usernames).collect()
}

pub fn load_followers(export_dir: &Path) -> HashSet<String> {
    let mut paths: Vec<_> = fs::read_dir(export_dir)
        .expect("failed to read export directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("followers_") && name.ends_with(".json"))
        })
        .collect();
    paths.sort();

    paths
        .iter()
        .flat_map(|path| {
            let contents = fs::read_to_string(path).expect("failed to read followers file");
            let entries: Vec<Value> =
                serde_json::from_str(&contents).expect("invalid followers JSON");
            usernames_from_string_list_data(&entries)
        })
        .collect()
}

pub fn load_following(export_dir: &Path) -> HashSet<String> {
    let path = export_dir.join("following.json");
    let contents = fs::read_to_string(&path).expect("failed to read following.json");
    let data: Value = serde_json::from_str(&contents).expect("invalid following.json");

    let entries = data
        .get("relationships_following")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    usernames_from_string_list_data(&entries)
}

pub fn label_value_username(entry: &Value) -> Option<String> {
    let empty = Vec::new();
    let label_values = entry
        .get("label_values")
        .and_then(Value::as_array)
        .unwrap_or(&empty);

    label_values
        .iter()
        .find(|label_value| label_value.get("label").and_then(Value::as_str) == Some("Username"))
        .and_then(|label_value| label_value.get("value").and_then(Value::as_str))
        .map(str::to_string)
}

fn usernames_from_label_values(entries: &[Value]) -> HashSet<String> {
    entries.iter().filter_map(label_value_username).collect()
}

pub fn load_pending_follow_requests(export_dir: &Path) -> HashSet<String> {
    let path = export_dir.join("pending_follow_requests.json");
    let contents = fs::read_to_string(&path).expect("failed to read pending_follow_requests.json");
    let entries: Vec<Value> =
        serde_json::from_str(&contents).expect("invalid pending_follow_requests.json");
    usernames_from_label_values(&entries)
}

pub fn load_close_friends(export_dir: &Path) -> HashSet<String> {
    let path = export_dir.join("close_friends.json");
    let contents = fs::read_to_string(&path).expect("failed to read close_friends.json");
    let entries: Vec<Value> = serde_json::from_str(&contents).expect("invalid close_friends.json");
    usernames_from_label_values(&entries)
}

pub fn is_deleted_account(username: &str) -> bool {
    username.starts_with("__deleted__")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        set, write_close_friends, write_followers, write_following, write_pending_follow_requests,
    };
    use serde_json::json;

    #[test]
    fn entry_usernames_reads_value_when_present() {
        let entry = json!({
            "string_list_data": [{"href": "https://www.instagram.com/foo", "value": "foo"}]
        });
        assert_eq!(entry_usernames(&entry), set(&["foo"]));
    }

    #[test]
    fn entry_usernames_falls_back_to_href_when_value_missing() {
        let entry = json!({
            "string_list_data": [{"href": "https://www.instagram.com/_u/bar"}]
        });
        assert_eq!(entry_usernames(&entry), set(&["bar"]));
    }

    #[test]
    fn entry_usernames_ignores_entries_without_string_list_data() {
        assert_eq!(entry_usernames(&json!({})), HashSet::new());
    }

    #[test]
    fn load_followers_reads_single_file() {
        let dir = tempfile::tempdir().unwrap();
        write_followers(dir.path(), &["alice", "bob"], "followers_1.json");
        assert_eq!(load_followers(dir.path()), set(&["alice", "bob"]));
    }

    #[test]
    fn load_followers_merges_paginated_files() {
        let dir = tempfile::tempdir().unwrap();
        write_followers(dir.path(), &["alice"], "followers_1.json");
        write_followers(dir.path(), &["bob"], "followers_2.json");
        assert_eq!(load_followers(dir.path()), set(&["alice", "bob"]));
    }

    #[test]
    fn load_following_reads_usernames() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["alice", "carol"]);
        assert_eq!(load_following(dir.path()), set(&["alice", "carol"]));
    }

    #[test]
    fn label_value_username_found() {
        let entry = json!({"label_values": [{"label": "Username", "value": "dave"}]});
        assert_eq!(label_value_username(&entry), Some("dave".to_string()));
    }

    #[test]
    fn label_value_username_missing() {
        let entry = json!({"label_values": [{"label": "Name", "value": "Dave"}]});
        assert_eq!(label_value_username(&entry), None);
    }

    #[test]
    fn load_pending_follow_requests_reads_usernames() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "erin"]);
        assert_eq!(
            load_pending_follow_requests(dir.path()),
            set(&["dave", "erin"])
        );
    }

    #[test]
    fn load_close_friends_reads_usernames() {
        let dir = tempfile::tempdir().unwrap();
        write_close_friends(dir.path(), &["frank", "grace"]);
        assert_eq!(load_close_friends(dir.path()), set(&["frank", "grace"]));
    }

    #[test]
    fn is_deleted_account_detects_deleted_marker() {
        assert!(is_deleted_account("__deleted__abc123"));
        assert!(!is_deleted_account("regular_user"));
    }
}

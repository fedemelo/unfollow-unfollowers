use std::collections::HashSet;
use std::fs;
use std::path::Path;

use chrono::{DateTime, NaiveDate};
use serde_json::Value;

use super::{DatedUsername, username_from_href};

fn epoch_to_date(epoch: i64) -> Option<NaiveDate> {
    DateTime::from_timestamp(epoch, 0).map(|dt| dt.naive_utc().date())
}

pub fn entry_usernames(entry: &Value) -> HashSet<String> {
    entry_dated_usernames(entry)
        .into_iter()
        .map(|dated| dated.username)
        .collect()
}

fn entry_dated_usernames(entry: &Value) -> Vec<DatedUsername> {
    let empty = Vec::new();
    let items = entry
        .get("string_list_data")
        .and_then(Value::as_array)
        .unwrap_or(&empty);

    items
        .iter()
        .map(|item| {
            let username = item
                .get("value")
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| {
                    let href = item.get("href").and_then(Value::as_str).unwrap_or("");
                    username_from_href(href)
                });
            let date = item
                .get("timestamp")
                .and_then(Value::as_i64)
                .and_then(epoch_to_date);
            DatedUsername { username, date }
        })
        .collect()
}

fn usernames_from_string_list_data(entries: &[Value]) -> HashSet<String> {
    entries.iter().flat_map(entry_usernames).collect()
}

fn dated_usernames_from_string_list_data(entries: &[Value]) -> Vec<DatedUsername> {
    entries.iter().flat_map(entry_dated_usernames).collect()
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

pub fn load_following(export_dir: &Path) -> Vec<DatedUsername> {
    let path = export_dir.join("following.json");
    let contents = fs::read_to_string(&path).expect("failed to read following.json");
    let data: Value = serde_json::from_str(&contents).expect("invalid following.json");

    let entries = data
        .get("relationships_following")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    dated_usernames_from_string_list_data(&entries)
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

pub fn load_pending_follow_requests(export_dir: &Path) -> Vec<DatedUsername> {
    let path = export_dir.join("pending_follow_requests.json");
    let contents = fs::read_to_string(&path).expect("failed to read pending_follow_requests.json");
    let entries: Vec<Value> =
        serde_json::from_str(&contents).expect("invalid pending_follow_requests.json");

    entries
        .iter()
        .filter_map(|entry| {
            label_value_username(entry).map(|username| DatedUsername {
                username,
                date: entry
                    .get("timestamp")
                    .and_then(Value::as_i64)
                    .and_then(epoch_to_date),
            })
        })
        .collect()
}

pub fn load_close_friends(export_dir: &Path) -> HashSet<String> {
    let path = export_dir.join("close_friends.json");
    let contents = fs::read_to_string(&path).expect("failed to read close_friends.json");
    let entries: Vec<Value> = serde_json::from_str(&contents).expect("invalid close_friends.json");
    usernames_from_label_values(&entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        set, write_close_friends, write_followers, write_following, write_pending_follow_requests,
        write_pending_follow_requests_with_timestamps,
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
        let usernames: HashSet<String> = load_following(dir.path())
            .into_iter()
            .map(|entry| entry.username)
            .collect();
        assert_eq!(usernames, set(&["alice", "carol"]));
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
        let usernames: HashSet<String> = load_pending_follow_requests(dir.path())
            .into_iter()
            .map(|request| request.username)
            .collect();
        assert_eq!(usernames, set(&["dave", "erin"]));
    }

    #[test]
    fn load_pending_follow_requests_reads_timestamps() {
        let dir = tempfile::tempdir().unwrap();
        let sent_at = NaiveDate::from_ymd_opt(2026, 8, 19)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();
        write_pending_follow_requests_with_timestamps(dir.path(), &[("dave", sent_at)]);

        let requests = load_pending_follow_requests(dir.path());
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].date,
            Some(NaiveDate::from_ymd_opt(2026, 8, 19).unwrap())
        );
    }

    #[test]
    fn load_close_friends_reads_usernames() {
        let dir = tempfile::tempdir().unwrap();
        write_close_friends(dir.path(), &["frank", "grace"]);
        assert_eq!(load_close_friends(dir.path()), set(&["frank", "grace"]));
    }
}

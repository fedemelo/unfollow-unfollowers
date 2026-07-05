use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

pub fn set(items: &[&str]) -> HashSet<String> {
    items.iter().map(|s| s.to_string()).collect()
}

pub fn write_followers(export_dir: &Path, usernames: &[&str], filename: &str) {
    let data: Vec<Value> = usernames
        .iter()
        .map(|username| {
            json!({
                "title": "",
                "media_list_data": [],
                "string_list_data": [{
                    "href": format!("https://www.instagram.com/{username}"),
                    "value": username,
                    "timestamp": 0,
                }],
            })
        })
        .collect();
    fs::write(
        export_dir.join(filename),
        serde_json::to_string(&data).unwrap(),
    )
    .unwrap();
}

pub fn write_following(export_dir: &Path, usernames: &[&str]) {
    let entries: Vec<Value> = usernames
        .iter()
        .map(|username| {
            json!({
                "title": username,
                "string_list_data": [{
                    "href": format!("https://www.instagram.com/_u/{username}"),
                    "timestamp": 0,
                }],
            })
        })
        .collect();
    let data = json!({ "relationships_following": entries });
    fs::write(
        export_dir.join("following.json"),
        serde_json::to_string(&data).unwrap(),
    )
    .unwrap();
}

fn label_value_entries(usernames: &[&str]) -> Vec<Value> {
    usernames
        .iter()
        .map(|username| {
            json!({
                "timestamp": 0,
                "media": [],
                "label_values": [
                    {"label": "URL", "value": ""},
                    {"label": "Name", "value": username},
                    {"label": "Username", "value": username},
                ],
                "fbid": "0",
            })
        })
        .collect()
}

pub fn write_pending_follow_requests(export_dir: &Path, usernames: &[&str]) {
    let data = label_value_entries(usernames);
    fs::write(
        export_dir.join("pending_follow_requests.json"),
        serde_json::to_string(&data).unwrap(),
    )
    .unwrap();
}

pub fn write_close_friends(export_dir: &Path, usernames: &[&str]) {
    let data = label_value_entries(usernames);
    fs::write(
        export_dir.join("close_friends.json"),
        serde_json::to_string(&data).unwrap(),
    )
    .unwrap();
}

pub fn write_username_list(path: &Path, lines: &[&str]) {
    fs::write(path, format!("{}\n", lines.join("\n"))).unwrap();
}

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;

use super::{DatedUsername, username_from_href};

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read {}", path.display()))
}

fn parse_instagram_date(text: &str) -> Option<NaiveDate> {
    NaiveDateTime::parse_from_str(text.trim(), "%b %e, %Y %I:%M %P")
        .map(|datetime| datetime.date())
        .ok()
}

/// Parses the `<a href="...">...</a></div><div>DATE</div>` blocks used by
/// `following.html` and `followers_*.html`.
fn parse_link_list(html: &str) -> Vec<(String, Option<NaiveDate>)> {
    let pattern =
        Regex::new(r#"<a target="_blank" href="([^"]+)"[^>]*>[^<]*</a></div><div>([^<]*)</div>"#)
            .expect("invalid link list regex");

    pattern
        .captures_iter(html)
        .map(|captures| {
            let username = username_from_href(&captures[1]);
            let sent_at = parse_instagram_date(&captures[2]);
            (username, sent_at)
        })
        .collect()
}

/// Parses the `<td>Username</td><td>USERNAME</td>...<div class="_3-94 _a6-o">DATE</div>` blocks
/// used by `pending_follow_requests.html` and `close_friends.html`.
fn parse_table_list(html: &str) -> Vec<(String, Option<NaiveDate>)> {
    let pattern = Regex::new(
        r#"<td class="_a6_q">Username</td><td class="_2piu _a6_r">([^<]+)</td></tr></table></div></div></div><div class="_3-94 _a6-o">([^<]+)</div>"#,
    )
    .expect("invalid table list regex");

    pattern
        .captures_iter(html)
        .map(|captures| {
            let username = captures[1].to_string();
            let sent_at = parse_instagram_date(&captures[2]);
            (username, sent_at)
        })
        .collect()
}

pub fn load_followers(export_dir: &Path) -> HashSet<String> {
    let mut paths: Vec<_> = fs::read_dir(export_dir)
        .expect("failed to read export directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("followers_") && name.ends_with(".html"))
        })
        .collect();
    paths.sort();

    paths
        .iter()
        .flat_map(|path| parse_link_list(&read_file(path)))
        .map(|(username, _)| username)
        .collect()
}

pub fn load_following(export_dir: &Path) -> Vec<DatedUsername> {
    let html = read_file(&export_dir.join("following.html"));
    parse_link_list(&html)
        .into_iter()
        .map(|(username, date)| DatedUsername { username, date })
        .collect()
}

pub fn load_pending_follow_requests(export_dir: &Path) -> Vec<DatedUsername> {
    let html = read_file(&export_dir.join("pending_follow_requests.html"));
    parse_table_list(&html)
        .into_iter()
        .map(|(username, date)| DatedUsername { username, date })
        .collect()
}

pub fn load_close_friends(export_dir: &Path) -> HashSet<String> {
    let html = read_file(&export_dir.join("close_friends.html"));
    parse_table_list(&html)
        .into_iter()
        .map(|(username, _)| username)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::set;

    fn link_block(username: &str, href: &str, date: &str) -> String {
        format!(
            r#"<div class="pam _3-95 _2ph- _a6-g uiBoxWhite noborder"><div class="_a6-p"><div><div><a target="_blank" href="{href}">{username}</a></div><div>{date}</div></div></div></div>"#
        )
    }

    fn table_block(username: &str, date: &str) -> String {
        format!(
            r#"<div class="pam _3-95 _2ph- _a6-g uiBoxWhite noborder"><div class="_3-95 _a6-p"><div class="pam _3-95 _2ph- _a6-g uiBoxWhite noborder"><div class="_a6-p"><table style="table-layout: fixed;"><tr><td class="_a6_q">Name</td><td class="_2piu _a6_r">{username}</td></tr><tr><td class="_a6_q">Username</td><td class="_2piu _a6_r">{username}</td></tr></table></div></div></div><div class="_3-94 _a6-o">{date}</div></div>"#
        )
    }

    #[test]
    fn parse_instagram_date_parses_short_and_double_digit_days() {
        assert_eq!(
            parse_instagram_date("Aug 9, 2026 8:24 am"),
            Some(NaiveDate::from_ymd_opt(2026, 8, 9).unwrap())
        );
        assert_eq!(
            parse_instagram_date("Aug 25, 2026 8:24 pm"),
            Some(NaiveDate::from_ymd_opt(2026, 8, 25).unwrap())
        );
    }

    #[test]
    fn load_following_reads_usernames_from_href() {
        let dir = tempfile::tempdir().unwrap();
        let html = format!(
            "<body>{}{}</body>",
            link_block(
                "https://www.instagram.com/_u/alice",
                "https://www.instagram.com/_u/alice",
                "Aug 25, 2026 9:14 am"
            ),
            link_block(
                "https://www.instagram.com/_u/bob",
                "https://www.instagram.com/_u/bob",
                "Aug 24, 2026 9:11 am"
            ),
        );
        fs::write(dir.path().join("following.html"), html).unwrap();
        let usernames: HashSet<String> = load_following(dir.path())
            .into_iter()
            .map(|entry| entry.username)
            .collect();
        assert_eq!(usernames, set(&["alice", "bob"]));
    }

    #[test]
    fn load_followers_merges_paginated_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("followers_1.html"),
            format!(
                "<body>{}</body>",
                link_block(
                    "alice",
                    "https://www.instagram.com/alice",
                    "Aug 25, 2026 8:59 am"
                )
            ),
        )
        .unwrap();
        fs::write(
            dir.path().join("followers_2.html"),
            format!(
                "<body>{}</body>",
                link_block(
                    "bob",
                    "https://www.instagram.com/bob",
                    "Aug 24, 2026 8:59 am"
                )
            ),
        )
        .unwrap();
        assert_eq!(load_followers(dir.path()), set(&["alice", "bob"]));
    }

    #[test]
    fn load_pending_follow_requests_reads_usernames_and_dates() {
        let dir = tempfile::tempdir().unwrap();
        let html = format!(
            "<body>{}{}</body>",
            table_block("dave", "Aug 19, 2026 8:24 am"),
            table_block("erin", "Aug 21, 2026 8:24 am"),
        );
        fs::write(dir.path().join("pending_follow_requests.html"), html).unwrap();

        let requests = load_pending_follow_requests(dir.path());
        let usernames: HashSet<String> = requests
            .iter()
            .map(|request| request.username.clone())
            .collect();
        assert_eq!(usernames, set(&["dave", "erin"]));

        let dave = requests.iter().find(|r| r.username == "dave").unwrap();
        assert_eq!(
            dave.date,
            Some(NaiveDate::from_ymd_opt(2026, 8, 19).unwrap())
        );
    }

    #[test]
    fn load_close_friends_reads_usernames() {
        let dir = tempfile::tempdir().unwrap();
        let html = format!(
            "<body>{}</body>",
            table_block("frank", "Aug 19, 2026 8:24 am")
        );
        fs::write(dir.path().join("close_friends.html"), html).unwrap();
        assert_eq!(load_close_friends(dir.path()), set(&["frank"]));
    }
}

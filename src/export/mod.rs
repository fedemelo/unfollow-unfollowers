mod html;
mod json;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use chrono::NaiveDate;

pub use json::{entry_usernames, label_value_username};

/// A username paired with the date of the event that produced it (a follow request sent, an
/// account followed) — used wherever a command supports excluding recent entries by date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatedUsername {
    pub username: String,
    pub date: Option<NaiveDate>,
}

impl DatedUsername {
    /// Whether this entry's date is on or before `cutoff` — or unknown.
    pub fn on_or_before(&self, cutoff: NaiveDate) -> bool {
        self.date.is_none_or(|date| date <= cutoff)
    }
}

/// Derives a username from a profile URL, e.g. `https://www.instagram.com/_u/alice/` -> `alice`.
fn username_from_href(href: &str) -> String {
    href.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string()
}

pub fn is_deleted_account(username: &str) -> bool {
    username.starts_with("__deleted__")
}

/// Picks the JSON or HTML loader for `base_name` depending on which file is present in
/// `export_dir` — the two export formats Instagram offers for a data download.
fn dispatch<T>(
    export_dir: &Path,
    base_name: &str,
    json_loader: impl FnOnce(&Path) -> T,
    html_loader: impl FnOnce(&Path) -> T,
) -> T {
    if export_dir.join(format!("{base_name}.json")).exists() {
        json_loader(export_dir)
    } else if export_dir.join(format!("{base_name}.html")).exists() {
        html_loader(export_dir)
    } else {
        panic!(
            "no {base_name}.json or {base_name}.html found in {} — point export_dir at your \
             unzipped Instagram export's connections/followers_and_following folder",
            export_dir.display()
        )
    }
}

fn has_followers_files(export_dir: &Path, extension: &str) -> bool {
    fs::read_dir(export_dir)
        .map(|entries| {
            entries.filter_map(|entry| entry.ok()).any(|entry| {
                entry
                    .path()
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("followers_") && name.ends_with(extension))
            })
        })
        .unwrap_or(false)
}

pub fn load_followers(export_dir: &Path) -> HashSet<String> {
    if has_followers_files(export_dir, ".json") {
        json::load_followers(export_dir)
    } else if has_followers_files(export_dir, ".html") {
        html::load_followers(export_dir)
    } else {
        panic!(
            "no followers_*.json or followers_*.html files found in {} — point export_dir at \
             your unzipped Instagram export's connections/followers_and_following folder",
            export_dir.display()
        )
    }
}

pub fn load_following(export_dir: &Path) -> Vec<DatedUsername> {
    dispatch(
        export_dir,
        "following",
        json::load_following,
        html::load_following,
    )
}

pub fn load_pending_follow_requests(export_dir: &Path) -> Vec<DatedUsername> {
    dispatch(
        export_dir,
        "pending_follow_requests",
        json::load_pending_follow_requests,
        html::load_pending_follow_requests,
    )
}

pub fn load_close_friends(export_dir: &Path) -> HashSet<String> {
    dispatch(
        export_dir,
        "close_friends",
        json::load_close_friends,
        html::load_close_friends,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_deleted_account_detects_deleted_marker() {
        assert!(is_deleted_account("__deleted__abc123"));
        assert!(!is_deleted_account("regular_user"));
    }

    #[test]
    fn dated_username_on_or_before_keeps_unknown_dates() {
        let entry = DatedUsername {
            username: "alice".to_string(),
            date: None,
        };
        assert!(entry.on_or_before(NaiveDate::from_ymd_opt(2026, 8, 20).unwrap()));
    }

    #[test]
    fn dated_username_on_or_before_compares_dates() {
        let entry = DatedUsername {
            username: "alice".to_string(),
            date: Some(NaiveDate::from_ymd_opt(2026, 8, 21).unwrap()),
        };
        assert!(!entry.on_or_before(NaiveDate::from_ymd_opt(2026, 8, 20).unwrap()));
        assert!(entry.on_or_before(NaiveDate::from_ymd_opt(2026, 8, 21).unwrap()));
    }

    #[test]
    fn dispatch_prefers_json_when_both_present() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("following.json"), "{}").unwrap();
        fs::write(dir.path().join("following.html"), "<html></html>").unwrap();
        let result = dispatch(dir.path(), "following", |_| "json", |_| "html");
        assert_eq!(result, "json");
    }

    #[test]
    fn dispatch_falls_back_to_html() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("following.html"), "<html></html>").unwrap();
        let result = dispatch(dir.path(), "following", |_| "json", |_| "html");
        assert_eq!(result, "html");
    }

    #[test]
    #[should_panic(expected = "no following.json or following.html")]
    fn dispatch_panics_when_neither_present() {
        let dir = tempfile::tempdir().unwrap();
        dispatch(dir.path(), "following", |_| "json", |_| "html");
    }

    #[test]
    fn load_followers_panics_when_no_followers_files_present() {
        let dir = tempfile::tempdir().unwrap();
        let result = std::panic::catch_unwind(|| load_followers(dir.path()));
        assert!(result.is_err());
    }
}

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use clap::Parser;

use crate::cli::DEFAULT_EXPORT_DIR;
use crate::export::load_close_friends;
use crate::formatting::save_report;
use crate::lists::load_username_list;

pub const DEFAULT_STANDARD_CLOSE_FRIENDS: &str = "data/standard_close_friends.txt";
pub const DEFAULT_OUTPUT: &str = "results/close_friends.txt";

pub fn diff_close_friends(
    export_dir: &Path,
    standard_path: &Path,
) -> (HashSet<String>, HashSet<String>) {
    let actual = load_close_friends(export_dir);
    let standard = load_username_list(standard_path);
    let unexpected = actual.difference(&standard).cloned().collect();
    let missing = standard.difference(&actual).cloned().collect();
    (unexpected, missing)
}

fn format_diff(unexpected: &HashSet<String>, missing: &HashSet<String>) -> String {
    if unexpected.is_empty() && missing.is_empty() {
        return "Close friends list matches your standard list.\n".to_string();
    }

    let mut report = String::new();

    let mut unexpected: Vec<&String> = unexpected.iter().collect();
    unexpected.sort_by_key(|username| username.to_lowercase());
    for username in unexpected {
        report.push_str(&format!("-{username}\n"));
    }

    let mut missing: Vec<&String> = missing.iter().collect();
    missing.sort_by_key(|username| username.to_lowercase());
    for username in missing {
        report.push_str(&format!("+{username}\n"));
    }

    report
}

#[derive(Parser)]
#[command(about = "Diff your actual close friends list against your standard (day-ones) list.")]
pub struct Args {
    /// Path to the unzipped 'followers_and_following' export folder
    #[arg(default_value = DEFAULT_EXPORT_DIR)]
    pub export_dir: PathBuf,

    /// Path to your hand-curated "day ones" list of usernames (one per line, # comments)
    #[arg(long, default_value = DEFAULT_STANDARD_CLOSE_FRIENDS)]
    pub standard: PathBuf,

    /// Where to save the results
    #[arg(long, default_value = DEFAULT_OUTPUT)]
    pub output: PathBuf,
}

pub fn run() {
    let args = Args::parse();
    let (unexpected, missing) = diff_close_friends(&args.export_dir, &args.standard);
    let report = format_diff(&unexpected, &missing);
    print!("{report}");
    save_report(&args.output, &report);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{set, write_close_friends, write_username_list};

    #[test]
    fn diff_close_friends_no_differences() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_close_friends(export_dir.path(), &["alice", "bob"]);
        let standard = tmp.path().join("standard.txt");
        write_username_list(&standard, &["alice", "bob"]);

        let (unexpected, missing) = diff_close_friends(export_dir.path(), &standard);
        assert_eq!(unexpected, HashSet::new());
        assert_eq!(missing, HashSet::new());
    }

    #[test]
    fn diff_close_friends_reports_unexpected_additions() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_close_friends(export_dir.path(), &["alice", "bob"]);
        let standard = tmp.path().join("standard.txt");
        write_username_list(&standard, &["alice"]);

        let (unexpected, missing) = diff_close_friends(export_dir.path(), &standard);
        assert_eq!(unexpected, set(&["bob"]));
        assert_eq!(missing, HashSet::new());
    }

    #[test]
    fn diff_close_friends_reports_missing_entries() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_close_friends(export_dir.path(), &["alice"]);
        let standard = tmp.path().join("standard.txt");
        write_username_list(&standard, &["alice", "carol"]);

        let (unexpected, missing) = diff_close_friends(export_dir.path(), &standard);
        assert_eq!(unexpected, HashSet::new());
        assert_eq!(missing, set(&["carol"]));
    }

    #[test]
    fn diff_close_friends_missing_standard_file_treats_all_as_unexpected() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_close_friends(export_dir.path(), &["alice"]);
        let standard = tmp.path().join("missing_standard.txt");

        let (unexpected, missing) = diff_close_friends(export_dir.path(), &standard);
        assert_eq!(unexpected, set(&["alice"]));
        assert_eq!(missing, HashSet::new());
    }
}

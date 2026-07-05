use std::collections::HashSet;
use std::path::{Path, PathBuf};

use clap::Parser;

use crate::cli::DEFAULT_EXPORT_DIR;
use crate::export::load_close_friends;
use crate::lists::load_username_list;

pub const DEFAULT_STANDARD_CLOSE_FRIENDS: &str = "data/standard_close_friends.txt";

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

#[derive(Parser)]
#[command(about = "Diff your actual close friends list against your standard (day-ones) list.")]
pub struct Args {
    #[arg(default_value = DEFAULT_EXPORT_DIR)]
    pub export_dir: PathBuf,

    #[arg(long, default_value = DEFAULT_STANDARD_CLOSE_FRIENDS)]
    pub standard: PathBuf,
}

pub fn run() {
    let args = Args::parse();
    let (unexpected, missing) = diff_close_friends(&args.export_dir, &args.standard);

    if unexpected.is_empty() && missing.is_empty() {
        println!("Close friends list matches your standard list.");
        return;
    }

    let mut unexpected: Vec<&String> = unexpected.iter().collect();
    unexpected.sort_by_key(|username| username.to_lowercase());
    for username in unexpected {
        println!("-{username}");
    }

    let mut missing: Vec<&String> = missing.iter().collect();
    missing.sort_by_key(|username| username.to_lowercase());
    for username in missing {
        println!("+{username}");
    }
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

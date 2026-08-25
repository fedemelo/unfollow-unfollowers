use std::collections::HashSet;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use clap::Parser;

use crate::cli::CommonArgs;
use crate::export::{load_followers, load_following};
use crate::formatting::{describe_with_cutoff, format_usernames, save_report};
use crate::lists::apply_manual_filters;

pub const DEFAULT_OUTPUT: &str = "results/non_followers.txt";

pub fn find_non_followers(
    export_dir: &Path,
    exclusions: &HashSet<String>,
    known_disabled: &HashSet<String>,
    exclude_after: Option<NaiveDate>,
) -> HashSet<String> {
    let following: HashSet<String> = load_following(export_dir)
        .into_iter()
        .filter(|entry| exclude_after.is_none_or(|cutoff| entry.on_or_before(cutoff)))
        .map(|entry| entry.username)
        .collect();
    let followers = load_followers(export_dir);
    let non_followers: HashSet<String> = following.difference(&followers).cloned().collect();
    apply_manual_filters(&non_followers, exclusions, known_disabled)
}

#[derive(Parser)]
#[command(about = "List Instagram accounts you follow that don't follow you back.")]
pub struct Args {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Exclude accounts you started following after this date (YYYY-MM-DD) — e.g. accounts too
    /// recently followed to realistically expect a follow-back yet
    #[arg(long = "exclude-after")]
    pub exclude_after: Option<NaiveDate>,

    /// Where to save the results
    #[arg(long, default_value = DEFAULT_OUTPUT)]
    pub output: PathBuf,
}

pub fn run() {
    let args = Args::parse();
    let (exclusions, known_disabled) = args.common.resolve_filters();
    let non_followers = find_non_followers(
        &args.common.export_dir,
        &exclusions,
        &known_disabled,
        args.exclude_after,
    );
    let description = describe_with_cutoff(
        "accounts you follow don't follow you back",
        args.exclude_after,
    );
    let report = format_usernames(&non_followers, &description);
    print!("{report}");
    save_report(&args.output, &report);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        set, write_followers, write_following, write_following_with_timestamps,
    };

    #[test]
    fn find_non_followers_returns_following_minus_followers() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["alice", "bob", "carol"]);
        write_followers(dir.path(), &["alice"], "followers_1.json");
        let result = find_non_followers(dir.path(), &HashSet::new(), &HashSet::new(), None);
        assert_eq!(result, set(&["bob", "carol"]));
    }

    #[test]
    fn find_non_followers_excludes_deleted_accounts() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["bob", "__deleted__abc123"]);
        write_followers(dir.path(), &[], "followers_1.json");
        let result = find_non_followers(dir.path(), &HashSet::new(), &HashSet::new(), None);
        assert_eq!(result, set(&["bob"]));
    }

    #[test]
    fn find_non_followers_applies_exclusions_and_known_disabled() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["bob", "carol", "dave"]);
        write_followers(dir.path(), &[], "followers_1.json");
        let result = find_non_followers(dir.path(), &set(&["bob"]), &set(&["carol"]), None);
        assert_eq!(result, set(&["dave"]));
    }

    #[test]
    fn find_non_followers_excludes_accounts_followed_after_cutoff() {
        let dir = tempfile::tempdir().unwrap();
        let before_cutoff = NaiveDate::from_ymd_opt(2026, 8, 19)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();
        let after_cutoff = NaiveDate::from_ymd_opt(2026, 8, 21)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp();
        write_following_with_timestamps(
            dir.path(),
            &[("bob", before_cutoff), ("carol", after_cutoff)],
        );
        write_followers(dir.path(), &[], "followers_1.json");

        let cutoff = NaiveDate::from_ymd_opt(2026, 8, 20).unwrap();
        let result = find_non_followers(dir.path(), &HashSet::new(), &HashSet::new(), Some(cutoff));
        assert_eq!(result, set(&["bob"]));
    }
}

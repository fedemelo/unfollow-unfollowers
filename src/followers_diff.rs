use std::collections::HashSet;
use std::path::{Path, PathBuf};

use clap::Parser;

use crate::cli::DEFAULT_EXPORT_DIR;
use crate::export::{load_followers, load_following};
use crate::formatting::{format_diff, save_report};
use crate::lists::{load_username_list, save_username_list};

pub const DEFAULT_FOLLOWERS_SNAPSHOT: &str = "data/snapshot_followers.txt";
pub const DEFAULT_FOLLOWING_SNAPSHOT: &str = "data/snapshot_following.txt";
pub const DEFAULT_FOLLOWERS_OUTPUT: &str = "results/followers_diff.txt";
pub const DEFAULT_FOLLOWING_OUTPUT: &str = "results/following_diff.txt";

/// Splits `current` against `previous` into what's new (in `current` only) and what's gone
/// (in `previous` only).
pub fn diff_sets(
    previous: &HashSet<String>,
    current: &HashSet<String>,
) -> (HashSet<String>, HashSet<String>) {
    let gained = current.difference(previous).cloned().collect();
    let lost = previous.difference(current).cloned().collect();
    (gained, lost)
}

/// Diffs `current` against whatever snapshot is saved at `snapshot_path`, then overwrites that
/// snapshot with `current` so the next run diffs against this one. Returns `None` (and writes no
/// report) the first time — with no prior snapshot, there's nothing meaningful to diff yet.
fn process_snapshot(
    label: &str,
    snapshot_path: &Path,
    current: &HashSet<String>,
    output_path: &Path,
    gained_label: &str,
    lost_label: &str,
    empty_message: &str,
) {
    if snapshot_path.exists() {
        let previous = load_username_list(snapshot_path);
        let (gained, lost) = diff_sets(&previous, current);
        let report = format_diff(&gained, &lost, gained_label, lost_label, empty_message);
        print!("{report}");
        save_report(output_path, &report);
    } else {
        println!(
            "No previous {label} snapshot found — saving the current list as the baseline for next time."
        );
    }
    save_username_list(snapshot_path, current);
}

#[derive(Parser)]
#[command(about = "Diff current followers/following against the snapshot saved on the last run.")]
pub struct Args {
    /// Path to the unzipped 'followers_and_following' export folder
    #[arg(default_value = DEFAULT_EXPORT_DIR)]
    pub export_dir: PathBuf,

    /// Path to the saved followers snapshot from the last run
    #[arg(long, default_value = DEFAULT_FOLLOWERS_SNAPSHOT)]
    pub followers_snapshot: PathBuf,

    /// Path to the saved following snapshot from the last run
    #[arg(long, default_value = DEFAULT_FOLLOWING_SNAPSHOT)]
    pub following_snapshot: PathBuf,

    /// Where to save the followers diff report
    #[arg(long, default_value = DEFAULT_FOLLOWERS_OUTPUT)]
    pub followers_output: PathBuf,

    /// Where to save the following diff report
    #[arg(long, default_value = DEFAULT_FOLLOWING_OUTPUT)]
    pub following_output: PathBuf,
}

pub fn run() {
    let args = Args::parse();

    let followers = load_followers(&args.export_dir);
    let following: HashSet<String> = load_following(&args.export_dir)
        .into_iter()
        .map(|entry| entry.username)
        .collect();

    process_snapshot(
        "followers",
        &args.followers_snapshot,
        &followers,
        &args.followers_output,
        "started following you since the last check",
        "stopped following you since the last check",
        "No changes in your followers since the last check.",
    );

    process_snapshot(
        "following",
        &args.following_snapshot,
        &following,
        &args.following_output,
        "you started following since the last check",
        "you unfollowed since the last check",
        "No changes in who you follow since the last check.",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::set;

    #[test]
    fn diff_sets_returns_gained_and_lost() {
        let previous = set(&["alice", "bob"]);
        let current = set(&["bob", "carol"]);
        let (gained, lost) = diff_sets(&previous, &current);
        assert_eq!(gained, set(&["carol"]));
        assert_eq!(lost, set(&["alice"]));
    }

    #[test]
    fn diff_sets_returns_empty_when_unchanged() {
        let previous = set(&["alice", "bob"]);
        let current = set(&["alice", "bob"]);
        let (gained, lost) = diff_sets(&previous, &current);
        assert_eq!(gained, HashSet::new());
        assert_eq!(lost, HashSet::new());
    }

    #[test]
    fn process_snapshot_writes_no_report_and_saves_baseline_on_first_run() {
        let dir = tempfile::tempdir().unwrap();
        let snapshot_path = dir.path().join("snapshot.txt");
        let output_path = dir.path().join("report.txt");

        process_snapshot(
            "followers",
            &snapshot_path,
            &set(&["alice"]),
            &output_path,
            "gained",
            "lost",
            "no changes",
        );

        assert!(!output_path.exists());
        assert_eq!(load_username_list(&snapshot_path), set(&["alice"]));
    }

    #[test]
    fn process_snapshot_writes_diff_report_and_updates_snapshot_on_later_run() {
        let dir = tempfile::tempdir().unwrap();
        let snapshot_path = dir.path().join("snapshot.txt");
        let output_path = dir.path().join("report.txt");
        save_username_list(&snapshot_path, &set(&["alice", "bob"]));

        process_snapshot(
            "followers",
            &snapshot_path,
            &set(&["bob", "carol"]),
            &output_path,
            "gained",
            "lost",
            "no changes",
        );

        let report = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(report, "+ gained\n- lost\n\n-alice\n+carol\n");
        assert_eq!(load_username_list(&snapshot_path), set(&["bob", "carol"]));
    }

    #[test]
    fn process_snapshot_reports_empty_message_when_no_changes() {
        let dir = tempfile::tempdir().unwrap();
        let snapshot_path = dir.path().join("snapshot.txt");
        let output_path = dir.path().join("report.txt");
        save_username_list(&snapshot_path, &set(&["alice"]));

        process_snapshot(
            "followers",
            &snapshot_path,
            &set(&["alice"]),
            &output_path,
            "gained",
            "lost",
            "no changes",
        );

        let report = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(report, "no changes\n");
    }
}

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use clap::Parser;

use crate::cli::CommonArgs;
use crate::export::load_pending_follow_requests;
use crate::formatting::{describe_with_cutoff, format_usernames, save_report};
use crate::lists::apply_manual_filters;

pub const DEFAULT_OUTPUT: &str = "results/pending_requests.txt";

pub fn find_pending_requests(
    export_dir: &Path,
    exclusions: &HashSet<String>,
    known_disabled: &HashSet<String>,
    exclude_after: Option<NaiveDate>,
) -> HashSet<String> {
    let pending: HashSet<String> = load_pending_follow_requests(export_dir)
        .into_iter()
        .filter(|request| exclude_after.is_none_or(|cutoff| request.on_or_before(cutoff)))
        .map(|request| request.username)
        .collect();
    apply_manual_filters(&pending, exclusions, known_disabled)
}

#[derive(Parser)]
#[command(about = "List sent follow requests that haven't been accepted yet.")]
pub struct Args {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Exclude requests sent after this date (YYYY-MM-DD) — e.g. requests too recent to
    /// realistically expect a reply yet
    #[arg(long = "exclude-after")]
    pub exclude_after: Option<NaiveDate>,

    /// Where to save the results
    #[arg(long, default_value = DEFAULT_OUTPUT)]
    pub output: PathBuf,
}

pub fn run() {
    let args = Args::parse();
    let (exclusions, known_disabled) = args.common.resolve_filters();
    let pending = find_pending_requests(
        &args.common.export_dir,
        &exclusions,
        &known_disabled,
        args.exclude_after,
    );
    let description =
        describe_with_cutoff("pending follow requests you've sent", args.exclude_after);
    let report = format_usernames(&pending, &description);
    print!("{report}");
    save_report(&args.output, &report);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        set, write_pending_follow_requests, write_pending_follow_requests_with_timestamps,
    };

    #[test]
    fn find_pending_requests_returns_all_pending() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "erin"]);
        let result = find_pending_requests(dir.path(), &HashSet::new(), &HashSet::new(), None);
        assert_eq!(result, set(&["dave", "erin"]));
    }

    #[test]
    fn find_pending_requests_excludes_deleted_accounts() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "__deleted__abc123"]);
        let result = find_pending_requests(dir.path(), &HashSet::new(), &HashSet::new(), None);
        assert_eq!(result, set(&["dave"]));
    }

    #[test]
    fn find_pending_requests_applies_exclusions_and_known_disabled() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "erin", "frank"]);
        let result = find_pending_requests(dir.path(), &set(&["dave"]), &set(&["erin"]), None);
        assert_eq!(result, set(&["frank"]));
    }

    #[test]
    fn find_pending_requests_excludes_requests_sent_after_cutoff() {
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
        write_pending_follow_requests_with_timestamps(
            dir.path(),
            &[("dave", before_cutoff), ("erin", after_cutoff)],
        );

        let cutoff = NaiveDate::from_ymd_opt(2026, 8, 20).unwrap();
        let result =
            find_pending_requests(dir.path(), &HashSet::new(), &HashSet::new(), Some(cutoff));
        assert_eq!(result, set(&["dave"]));
    }

    #[test]
    fn find_pending_requests_keeps_unknown_send_dates_regardless_of_cutoff() {
        let dir = tempfile::tempdir().unwrap();
        let entries = serde_json::json!([{
            "label_values": [{"label": "Username", "value": "dave"}],
        }]);
        std::fs::write(
            dir.path().join("pending_follow_requests.json"),
            entries.to_string(),
        )
        .unwrap();

        let cutoff = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let result =
            find_pending_requests(dir.path(), &HashSet::new(), &HashSet::new(), Some(cutoff));
        assert_eq!(result, set(&["dave"]));
    }
}

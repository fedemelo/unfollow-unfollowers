use std::collections::HashSet;
use std::path::Path;

use clap::Parser;

use crate::cli::CommonArgs;
use crate::export::load_pending_follow_requests;
use crate::formatting::print_usernames;
use crate::lists::apply_manual_filters;

pub fn find_pending_requests(
    export_dir: &Path,
    exclusions: &HashSet<String>,
    known_disabled: &HashSet<String>,
) -> HashSet<String> {
    let pending = load_pending_follow_requests(export_dir);
    apply_manual_filters(&pending, exclusions, known_disabled)
}

#[derive(Parser)]
#[command(about = "List sent follow requests that haven't been accepted yet.")]
pub struct Args {
    #[command(flatten)]
    pub common: CommonArgs,
}

pub fn run() {
    let args = Args::parse();
    let (exclusions, known_disabled) = args.common.resolve_filters();
    let pending = find_pending_requests(&args.common.export_dir, &exclusions, &known_disabled);
    print_usernames(&pending, "pending follow requests you've sent");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{set, write_pending_follow_requests};

    #[test]
    fn find_pending_requests_returns_all_pending() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "erin"]);
        let result = find_pending_requests(dir.path(), &HashSet::new(), &HashSet::new());
        assert_eq!(result, set(&["dave", "erin"]));
    }

    #[test]
    fn find_pending_requests_excludes_deleted_accounts() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "__deleted__abc123"]);
        let result = find_pending_requests(dir.path(), &HashSet::new(), &HashSet::new());
        assert_eq!(result, set(&["dave"]));
    }

    #[test]
    fn find_pending_requests_applies_exclusions_and_known_disabled() {
        let dir = tempfile::tempdir().unwrap();
        write_pending_follow_requests(dir.path(), &["dave", "erin", "frank"]);
        let result = find_pending_requests(dir.path(), &set(&["dave"]), &set(&["erin"]));
        assert_eq!(result, set(&["frank"]));
    }
}

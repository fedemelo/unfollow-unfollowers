use std::collections::HashSet;
use std::path::Path;

use clap::Parser;

use crate::cli::CommonArgs;
use crate::export::{load_followers, load_following};
use crate::formatting::print_usernames;
use crate::lists::apply_manual_filters;

pub fn find_non_followers(
    export_dir: &Path,
    exclusions: &HashSet<String>,
    known_disabled: &HashSet<String>,
) -> HashSet<String> {
    let following = load_following(export_dir);
    let followers = load_followers(export_dir);
    let non_followers: HashSet<String> = following.difference(&followers).cloned().collect();
    apply_manual_filters(&non_followers, exclusions, known_disabled)
}

#[derive(Parser)]
#[command(about = "List Instagram accounts you follow that don't follow you back.")]
pub struct Args {
    #[command(flatten)]
    pub common: CommonArgs,
}

pub fn run() {
    let args = Args::parse();
    let (exclusions, known_disabled) = args.common.resolve_filters();
    let non_followers = find_non_followers(&args.common.export_dir, &exclusions, &known_disabled);
    print_usernames(&non_followers, "accounts you follow don't follow you back");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{set, write_followers, write_following};

    #[test]
    fn find_non_followers_returns_following_minus_followers() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["alice", "bob", "carol"]);
        write_followers(dir.path(), &["alice"], "followers_1.json");
        let result = find_non_followers(dir.path(), &HashSet::new(), &HashSet::new());
        assert_eq!(result, set(&["bob", "carol"]));
    }

    #[test]
    fn find_non_followers_excludes_deleted_accounts() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["bob", "__deleted__abc123"]);
        write_followers(dir.path(), &[], "followers_1.json");
        let result = find_non_followers(dir.path(), &HashSet::new(), &HashSet::new());
        assert_eq!(result, set(&["bob"]));
    }

    #[test]
    fn find_non_followers_applies_exclusions_and_known_disabled() {
        let dir = tempfile::tempdir().unwrap();
        write_following(dir.path(), &["bob", "carol", "dave"]);
        write_followers(dir.path(), &[], "followers_1.json");
        let result = find_non_followers(dir.path(), &set(&["bob"]), &set(&["carol"]));
        assert_eq!(result, set(&["dave"]));
    }
}

use std::collections::HashSet;
use std::path::PathBuf;

use clap::Args;

use crate::lists::load_username_list;

pub const DEFAULT_EXPORT_DIR: &str = "export/connections/followers_and_following";
pub const DEFAULT_EXCLUSIONS: &str = "data/excluded_accounts.txt";
pub const DEFAULT_KNOWN_DISABLED: &str = "data/known_disabled_accounts.txt";

#[derive(Args, Debug)]
pub struct CommonArgs {
    /// Path to the unzipped 'followers_and_following' export folder
    #[arg(default_value = DEFAULT_EXPORT_DIR)]
    pub export_dir: PathBuf,

    /// Path to a text file of usernames (one per line, # comments) to never unfollow
    #[arg(long, default_value = DEFAULT_EXCLUSIONS)]
    pub exclusions: PathBuf,

    /// Path to a text file of usernames previously confirmed by hand to be deleted/disabled
    #[arg(long = "known-disabled", default_value = DEFAULT_KNOWN_DISABLED)]
    pub known_disabled: PathBuf,

    /// Include accounts from --known-disabled, to manually re-verify whether they're back
    #[arg(long = "recheck-disabled")]
    pub recheck_disabled: bool,
}

impl CommonArgs {
    pub fn resolve_filters(&self) -> (HashSet<String>, HashSet<String>) {
        let exclusions = load_username_list(&self.exclusions);
        let known_disabled = if self.recheck_disabled {
            HashSet::new()
        } else {
            load_username_list(&self.known_disabled)
        };
        (exclusions, known_disabled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::fs;

    #[derive(Parser)]
    struct TestArgs {
        #[command(flatten)]
        common: CommonArgs,
    }

    #[test]
    fn defaults_match_expected_paths() {
        let args = TestArgs::parse_from(["prog"]);
        assert_eq!(args.common.export_dir, PathBuf::from(DEFAULT_EXPORT_DIR));
        assert_eq!(args.common.exclusions, PathBuf::from(DEFAULT_EXCLUSIONS));
        assert_eq!(
            args.common.known_disabled,
            PathBuf::from(DEFAULT_KNOWN_DISABLED)
        );
        assert!(!args.common.recheck_disabled);
    }

    #[test]
    fn resolve_filters_loads_exclusions_and_known_disabled() {
        let dir = tempfile::tempdir().unwrap();
        let exclusions_path = dir.path().join("exclusions.txt");
        let known_disabled_path = dir.path().join("known_disabled.txt");
        fs::write(&exclusions_path, "alice\n").unwrap();
        fs::write(&known_disabled_path, "bob\n").unwrap();

        let args = TestArgs::parse_from([
            "prog",
            "--exclusions",
            exclusions_path.to_str().unwrap(),
            "--known-disabled",
            known_disabled_path.to_str().unwrap(),
        ]);

        let (exclusions, known_disabled) = args.common.resolve_filters();
        assert_eq!(exclusions, HashSet::from(["alice".to_string()]));
        assert_eq!(known_disabled, HashSet::from(["bob".to_string()]));
    }

    #[test]
    fn resolve_filters_clears_known_disabled_when_rechecking() {
        let dir = tempfile::tempdir().unwrap();
        let known_disabled_path = dir.path().join("known_disabled.txt");
        fs::write(&known_disabled_path, "bob\n").unwrap();

        let args = TestArgs::parse_from([
            "prog",
            "--known-disabled",
            known_disabled_path.to_str().unwrap(),
            "--recheck-disabled",
        ]);

        let (_, known_disabled) = args.common.resolve_filters();
        assert_eq!(known_disabled, HashSet::new());
    }
}

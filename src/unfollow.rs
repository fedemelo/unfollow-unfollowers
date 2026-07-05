use std::path::{Path, PathBuf};

use clap::Parser;

use crate::cli::{DEFAULT_EXCLUSIONS, DEFAULT_EXPORT_DIR, DEFAULT_KNOWN_DISABLED};
use crate::mutate::{
    remove_from_following, remove_from_pending_requests, remove_from_username_list,
};

pub fn unfollow(
    username: &str,
    export_dir: &Path,
    exclusions_path: &Path,
    known_disabled_path: &Path,
) -> Vec<String> {
    let mut changes = Vec::new();
    if remove_from_following(export_dir, username) {
        changes.push("following.json".to_string());
    }
    if remove_from_pending_requests(export_dir, username) {
        changes.push("pending_follow_requests.json".to_string());
    }
    if remove_from_username_list(exclusions_path, username) {
        changes.push(exclusions_path.display().to_string());
    }
    if remove_from_username_list(known_disabled_path, username) {
        changes.push(known_disabled_path.display().to_string());
    }
    changes
}

#[derive(Parser)]
#[command(about = "Remove a username you've just unfollowed from the local export and lists.")]
pub struct Args {
    pub username: String,

    #[arg(long = "export-dir", default_value = DEFAULT_EXPORT_DIR)]
    pub export_dir: PathBuf,

    #[arg(long, default_value = DEFAULT_EXCLUSIONS)]
    pub exclusions: PathBuf,

    #[arg(long = "known-disabled", default_value = DEFAULT_KNOWN_DISABLED)]
    pub known_disabled: PathBuf,
}

pub fn run() {
    let args = Args::parse();
    let changes = unfollow(
        &args.username,
        &args.export_dir,
        &args.exclusions,
        &args.known_disabled,
    );
    if changes.is_empty() {
        println!("{} not found anywhere.", args.username);
    } else {
        println!("Removed {} from: {}", args.username, changes.join(", "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        write_following, write_pending_follow_requests, write_username_list,
    };

    #[test]
    fn unfollow_removes_from_following() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_following(export_dir.path(), &["alice", "bob"]);
        let exclusions = tmp.path().join("excluded.txt");
        let known_disabled = tmp.path().join("known_disabled.txt");

        let changes = unfollow("alice", export_dir.path(), &exclusions, &known_disabled);
        assert_eq!(changes, vec!["following.json".to_string()]);
    }

    #[test]
    fn unfollow_removes_from_pending_requests() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_pending_follow_requests(export_dir.path(), &["dave"]);
        let exclusions = tmp.path().join("excluded.txt");
        let known_disabled = tmp.path().join("known_disabled.txt");

        let changes = unfollow("dave", export_dir.path(), &exclusions, &known_disabled);
        assert_eq!(changes, vec!["pending_follow_requests.json".to_string()]);
    }

    #[test]
    fn unfollow_removes_from_username_lists() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let exclusions = tmp.path().join("excluded.txt");
        let known_disabled = tmp.path().join("known_disabled.txt");
        write_username_list(&exclusions, &["alice"]);
        write_username_list(&known_disabled, &["alice"]);

        let changes = unfollow("alice", export_dir.path(), &exclusions, &known_disabled);
        assert_eq!(
            changes,
            vec![
                exclusions.display().to_string(),
                known_disabled.display().to_string(),
            ]
        );
    }

    #[test]
    fn unfollow_removes_from_everywhere_at_once() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_following(export_dir.path(), &["alice"]);
        write_pending_follow_requests(export_dir.path(), &["alice"]);
        let exclusions = tmp.path().join("excluded.txt");
        let known_disabled = tmp.path().join("known_disabled.txt");
        write_username_list(&exclusions, &["alice"]);
        write_username_list(&known_disabled, &["alice"]);

        let changes = unfollow("alice", export_dir.path(), &exclusions, &known_disabled);
        assert_eq!(
            changes,
            vec![
                "following.json".to_string(),
                "pending_follow_requests.json".to_string(),
                exclusions.display().to_string(),
                known_disabled.display().to_string(),
            ]
        );
    }

    #[test]
    fn unfollow_returns_empty_list_when_username_not_found() {
        let export_dir = tempfile::tempdir().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        write_following(export_dir.path(), &["bob"]);
        let exclusions = tmp.path().join("excluded.txt");
        let known_disabled = tmp.path().join("known_disabled.txt");

        let changes = unfollow("alice", export_dir.path(), &exclusions, &known_disabled);
        assert_eq!(changes, Vec::<String>::new());
    }
}

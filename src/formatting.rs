use std::collections::HashSet;
use std::fs;
use std::path::Path;

use chrono::NaiveDate;

pub fn profile_url(username: &str) -> String {
    format!("https://www.instagram.com/{username}")
}

/// Appends a note about the exclusion cutoff to a report description, if one was applied.
pub fn describe_with_cutoff(description: &str, exclude_after: Option<NaiveDate>) -> String {
    match exclude_after {
        Some(cutoff) => format!("{description}, excluding anything after {cutoff}"),
        None => description.to_string(),
    }
}

pub fn format_usernames(usernames: &HashSet<String>, description: &str) -> String {
    let mut usernames: Vec<&String> = usernames.iter().collect();
    usernames.sort_by_key(|username| username.to_lowercase());

    let mut report = format!("{} {}:\n\n", usernames.len(), description);
    for username in usernames {
        report.push_str(&profile_url(username));
        report.push('\n');
    }
    report
}

pub fn print_usernames(usernames: &HashSet<String>, description: &str) {
    print!("{}", format_usernames(usernames, description));
}

/// Formats a plus/minus diff report, with a header spelling out what each sign means so the
/// convention never has to be inferred — `added` gets a leading `+`, `removed` a leading `-`.
pub fn format_diff(
    added: &HashSet<String>,
    removed: &HashSet<String>,
    added_label: &str,
    removed_label: &str,
    empty_message: &str,
) -> String {
    if added.is_empty() && removed.is_empty() {
        return format!("{empty_message}\n");
    }

    let mut report = format!("+ {added_label}\n- {removed_label}\n\n");

    let mut removed: Vec<&String> = removed.iter().collect();
    removed.sort_by_key(|username| username.to_lowercase());
    for username in removed {
        report.push_str(&format!("-{username}\n"));
    }

    let mut added: Vec<&String> = added.iter().collect();
    added.sort_by_key(|username| username.to_lowercase());
    for username in added {
        report.push_str(&format!("+{username}\n"));
    }

    report
}

/// Writes `report` to `path`, creating any missing parent directories, and prints where it went.
pub fn save_report(path: &Path, report: &str) {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(path, report).expect("failed to write output file");
    println!("Saved to {}", path.display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_url_builds_instagram_link() {
        assert_eq!(profile_url("alice"), "https://www.instagram.com/alice");
    }

    #[test]
    fn print_usernames_handles_empty_set() {
        print_usernames(&HashSet::new(), "accounts");
    }

    #[test]
    fn describe_with_cutoff_leaves_description_unchanged_when_none() {
        assert_eq!(describe_with_cutoff("accounts", None), "accounts");
    }

    #[test]
    fn describe_with_cutoff_appends_cutoff_note_when_some() {
        let cutoff = NaiveDate::from_ymd_opt(2026, 8, 20).unwrap();
        assert_eq!(
            describe_with_cutoff("accounts", Some(cutoff)),
            "accounts, excluding anything after 2026-08-20"
        );
    }

    #[test]
    fn format_usernames_sorts_case_insensitively_and_counts() {
        let usernames: HashSet<String> = ["Bob", "alice"].iter().map(|s| s.to_string()).collect();
        let report = format_usernames(&usernames, "accounts");
        assert_eq!(
            report,
            "2 accounts:\n\nhttps://www.instagram.com/alice\nhttps://www.instagram.com/Bob\n"
        );
    }

    #[test]
    fn save_report_creates_parent_directories_and_writes_contents() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("report.txt");
        save_report(&path, "hello");
        assert_eq!(fs::read_to_string(&path).unwrap(), "hello");
    }

    #[test]
    fn format_diff_reports_empty_message_when_no_changes() {
        let report = format_diff(
            &HashSet::new(),
            &HashSet::new(),
            "added",
            "removed",
            "Nothing changed.",
        );
        assert_eq!(report, "Nothing changed.\n");
    }

    #[test]
    fn format_diff_includes_legend_and_signed_sorted_lines() {
        let added: HashSet<String> = ["Bob".to_string()].into_iter().collect();
        let removed: HashSet<String> = ["alice".to_string()].into_iter().collect();
        let report = format_diff(&added, &removed, "gained", "lost", "Nothing changed.");
        assert_eq!(report, "+ gained\n- lost\n\n-alice\n+Bob\n");
    }
}

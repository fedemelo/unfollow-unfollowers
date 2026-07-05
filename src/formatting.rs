use std::collections::HashSet;

pub fn profile_url(username: &str) -> String {
    format!("https://www.instagram.com/{username}")
}

pub fn print_usernames(usernames: &HashSet<String>, description: &str) {
    let mut usernames: Vec<&String> = usernames.iter().collect();
    usernames.sort_by_key(|username| username.to_lowercase());

    println!("{} {}:\n", usernames.len(), description);
    for username in usernames {
        println!("{}", profile_url(username));
    }
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
}

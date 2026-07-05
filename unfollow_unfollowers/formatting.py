from collections.abc import Iterable


def profile_url(username: str) -> str:
    return f"https://www.instagram.com/{username}"


def print_usernames(usernames: Iterable[str], description: str) -> None:
    usernames = sorted(usernames, key=str.lower)
    print(f"{len(usernames)} {description}:\n")
    for username in usernames:
        print(profile_url(username))

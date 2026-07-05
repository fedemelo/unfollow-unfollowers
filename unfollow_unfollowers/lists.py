from pathlib import Path

from .export import is_deleted_account


def load_username_list(path: Path) -> set[str]:
    if not path.exists():
        return set()
    lines = path.read_text().splitlines()
    return {line.strip() for line in lines if line.strip() and not line.strip().startswith("#")}


def apply_manual_filters(
    usernames: set[str],
    exclusions: set[str] = frozenset(),
    known_disabled: set[str] = frozenset(),
) -> set[str]:
    return {
        username
        for username in usernames
        if not is_deleted_account(username)
        and username not in exclusions
        and username not in known_disabled
    }

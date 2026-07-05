import json
from pathlib import Path


def entry_usernames(entry: dict) -> set[str]:
    usernames = set()
    for item in entry.get("string_list_data", []):
        username = item.get("value") or item.get("href", "").rstrip("/").rsplit("/", 1)[-1]
        usernames.add(username)
    return usernames


def _usernames_from_string_list_data(entries: list[dict]) -> set[str]:
    usernames = set()
    for entry in entries:
        usernames |= entry_usernames(entry)
    return usernames


def load_followers(export_dir: Path) -> set[str]:
    usernames = set()
    for path in sorted(export_dir.glob("followers_*.json")):
        usernames |= _usernames_from_string_list_data(json.loads(path.read_text()))
    return usernames


def load_following(export_dir: Path) -> set[str]:
    data = json.loads((export_dir / "following.json").read_text())
    return _usernames_from_string_list_data(data["relationships_following"])


def is_deleted_account(username: str) -> bool:
    return username.startswith("__deleted__")

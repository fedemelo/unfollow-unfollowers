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


def label_value_username(entry: dict) -> str | None:
    for label_value in entry.get("label_values", []):
        if label_value.get("label") == "Username":
            return label_value["value"]
    return None


def _usernames_from_label_values(entries: list[dict]) -> set[str]:
    return {label_value_username(entry) for entry in entries} - {None}


def load_pending_follow_requests(export_dir: Path) -> set[str]:
    data = json.loads((export_dir / "pending_follow_requests.json").read_text())
    return _usernames_from_label_values(data)


def load_close_friends(export_dir: Path) -> set[str]:
    data = json.loads((export_dir / "close_friends.json").read_text())
    return _usernames_from_label_values(data)


def is_deleted_account(username: str) -> bool:
    return username.startswith("__deleted__")

import json
from pathlib import Path

from .export import entry_usernames, label_value_username


def _write_json(path: Path, data) -> None:
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")


def remove_from_following(export_dir: Path, username: str) -> bool:
    path = export_dir / "following.json"
    if not path.exists():
        return False
    data = json.loads(path.read_text())
    entries = data.get("relationships_following", [])
    filtered = [entry for entry in entries if username not in entry_usernames(entry)]
    if len(filtered) == len(entries):
        return False
    data["relationships_following"] = filtered
    _write_json(path, data)
    return True


def remove_from_pending_requests(export_dir: Path, username: str) -> bool:
    path = export_dir / "pending_follow_requests.json"
    if not path.exists():
        return False
    entries = json.loads(path.read_text())
    filtered = [entry for entry in entries if label_value_username(entry) != username]
    if len(filtered) == len(entries):
        return False
    _write_json(path, filtered)
    return True


def remove_from_username_list(path: Path, username: str) -> bool:
    if not path.exists():
        return False
    lines = path.read_text().splitlines()
    filtered = [line for line in lines if line.strip() != username]
    if len(filtered) == len(lines):
        return False
    path.write_text("\n".join(filtered) + "\n")
    return True

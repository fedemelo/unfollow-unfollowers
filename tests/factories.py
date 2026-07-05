import json
from pathlib import Path


def write_followers(
    export_dir: Path, usernames: list[str], filename: str = "followers_1.json"
) -> None:
    data = [
        {
            "title": "",
            "media_list_data": [],
            "string_list_data": [
                {"href": f"https://www.instagram.com/{username}", "value": username, "timestamp": 0}
            ],
        }
        for username in usernames
    ]
    (export_dir / filename).write_text(json.dumps(data))


def write_following(export_dir: Path, usernames: list[str]) -> None:
    data = {
        "relationships_following": [
            {
                "title": username,
                "string_list_data": [
                    {"href": f"https://www.instagram.com/_u/{username}", "timestamp": 0}
                ],
            }
            for username in usernames
        ]
    }
    (export_dir / "following.json").write_text(json.dumps(data))


def _label_value_entries(usernames: list[str]) -> list[dict]:
    return [
        {
            "timestamp": 0,
            "media": [],
            "label_values": [
                {"label": "URL", "value": ""},
                {"label": "Name", "value": username},
                {"label": "Username", "value": username},
            ],
            "fbid": "0",
        }
        for username in usernames
    ]


def write_pending_follow_requests(export_dir: Path, usernames: list[str]) -> None:
    data = _label_value_entries(usernames)
    (export_dir / "pending_follow_requests.json").write_text(json.dumps(data))


def write_close_friends(export_dir: Path, usernames: list[str]) -> None:
    data = _label_value_entries(usernames)
    (export_dir / "close_friends.json").write_text(json.dumps(data))


def write_username_list(path: Path, lines: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n")

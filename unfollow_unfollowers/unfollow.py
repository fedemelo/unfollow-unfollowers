import argparse
from pathlib import Path

from .cli import DEFAULT_EXCLUSIONS, DEFAULT_EXPORT_DIR, DEFAULT_KNOWN_DISABLED
from .mutate import remove_from_following, remove_from_pending_requests, remove_from_username_list


def unfollow(
    username: str,
    export_dir: Path,
    exclusions_path: Path,
    known_disabled_path: Path,
) -> list[str]:
    changes = []
    if remove_from_following(export_dir, username):
        changes.append("following.json")
    if remove_from_pending_requests(export_dir, username):
        changes.append("pending_follow_requests.json")
    if remove_from_username_list(exclusions_path, username):
        changes.append(str(exclusions_path))
    if remove_from_username_list(known_disabled_path, username):
        changes.append(str(known_disabled_path))
    return changes


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Remove a username you've just unfollowed from the local export and username lists."
    )
    parser.add_argument("username")
    parser.add_argument("--export-dir", type=Path, default=DEFAULT_EXPORT_DIR)
    parser.add_argument("--exclusions", type=Path, default=DEFAULT_EXCLUSIONS)
    parser.add_argument("--known-disabled", type=Path, default=DEFAULT_KNOWN_DISABLED)
    args = parser.parse_args()

    changes = unfollow(args.username, args.export_dir, args.exclusions, args.known_disabled)
    if changes:
        print(f"Removed {args.username} from: {', '.join(changes)}")
    else:
        print(f"{args.username} not found anywhere.")


if __name__ == "__main__":
    main()

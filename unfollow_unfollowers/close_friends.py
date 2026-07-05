import argparse
from pathlib import Path

from .cli import DEFAULT_EXPORT_DIR
from .export import load_close_friends
from .lists import load_username_list

DEFAULT_STANDARD_CLOSE_FRIENDS = Path("data/standard_close_friends.txt")


def diff_close_friends(export_dir: Path, standard_path: Path) -> tuple[set[str], set[str]]:
    actual = load_close_friends(export_dir)
    standard = load_username_list(standard_path)
    unexpected = actual - standard
    missing = standard - actual
    return unexpected, missing


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Diff your actual close friends list against your standard (day-ones) list."
    )
    parser.add_argument("export_dir", type=Path, nargs="?", default=DEFAULT_EXPORT_DIR)
    parser.add_argument("--standard", type=Path, default=DEFAULT_STANDARD_CLOSE_FRIENDS)
    args = parser.parse_args()

    unexpected, missing = diff_close_friends(args.export_dir, args.standard)

    if not unexpected and not missing:
        print("Close friends list matches your standard list.")
        return

    for username in sorted(unexpected, key=str.lower):
        print(f"-{username}")
    for username in sorted(missing, key=str.lower):
        print(f"+{username}")


if __name__ == "__main__":
    main()

from pathlib import Path

from . import cli
from .export import load_followers, load_following
from .formatting import print_usernames
from .lists import apply_manual_filters


def find_non_followers(
    export_dir: Path,
    exclusions: set[str] = frozenset(),
    known_disabled: set[str] = frozenset(),
) -> set[str]:
    non_followers = load_following(export_dir) - load_followers(export_dir)
    return apply_manual_filters(non_followers, exclusions, known_disabled)


def main() -> None:
    parser = cli.build_parser("List Instagram accounts you follow that don't follow you back.")
    args = parser.parse_args()
    exclusions, known_disabled = cli.resolve_filters(args)

    non_followers = find_non_followers(args.export_dir, exclusions, known_disabled)
    print_usernames(non_followers, "accounts you follow don't follow you back")


if __name__ == "__main__":
    main()

import argparse
from pathlib import Path

from .lists import load_username_list

DEFAULT_EXPORT_DIR = Path("export/connections/followers_and_following")
DEFAULT_EXCLUSIONS = Path("data/excluded_accounts.txt")
DEFAULT_KNOWN_DISABLED = Path("data/known_disabled_accounts.txt")


def build_parser(description: str) -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument(
        "export_dir",
        type=Path,
        nargs="?",
        default=DEFAULT_EXPORT_DIR,
        help="Path to the unzipped 'followers_and_following' export folder",
    )
    parser.add_argument(
        "--exclusions",
        type=Path,
        default=DEFAULT_EXCLUSIONS,
        help="Path to a text file of usernames (one per line, # comments) to never unfollow",
    )
    parser.add_argument(
        "--known-disabled",
        type=Path,
        default=DEFAULT_KNOWN_DISABLED,
        help="Path to a text file of usernames previously confirmed by hand to be deleted/disabled",
    )
    parser.add_argument(
        "--recheck-disabled",
        action="store_true",
        help="Include accounts from --known-disabled, to manually re-verify whether they're back",
    )
    return parser


def resolve_filters(args: argparse.Namespace) -> tuple[set[str], set[str]]:
    exclusions = load_username_list(args.exclusions)
    known_disabled = set() if args.recheck_disabled else load_username_list(args.known_disabled)
    return exclusions, known_disabled

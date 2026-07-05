from pathlib import Path

from . import cli
from .export import load_pending_follow_requests
from .formatting import print_usernames
from .lists import apply_manual_filters


def find_pending_requests(
    export_dir: Path,
    exclusions: set[str] = frozenset(),
    known_disabled: set[str] = frozenset(),
) -> set[str]:
    pending = load_pending_follow_requests(export_dir)
    return apply_manual_filters(pending, exclusions, known_disabled)


def main() -> None:
    parser = cli.build_parser("List sent follow requests that haven't been accepted yet.")
    args = parser.parse_args()
    exclusions, known_disabled = cli.resolve_filters(args)

    pending = find_pending_requests(args.export_dir, exclusions, known_disabled)
    print_usernames(pending, "pending follow requests you've sent")


if __name__ == "__main__":
    main()

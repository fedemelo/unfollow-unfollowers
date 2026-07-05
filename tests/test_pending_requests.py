from unfollow_unfollowers.pending_requests import find_pending_requests

from .factories import write_pending_follow_requests


def test_find_pending_requests_returns_all_pending(export_dir):
    write_pending_follow_requests(export_dir, ["dave", "erin"])
    assert find_pending_requests(export_dir) == {"dave", "erin"}


def test_find_pending_requests_excludes_deleted_accounts(export_dir):
    write_pending_follow_requests(export_dir, ["dave", "__deleted__abc123"])
    assert find_pending_requests(export_dir) == {"dave"}


def test_find_pending_requests_applies_exclusions_and_known_disabled(export_dir):
    write_pending_follow_requests(export_dir, ["dave", "erin", "frank"])
    result = find_pending_requests(export_dir, exclusions={"dave"}, known_disabled={"erin"})
    assert result == {"frank"}

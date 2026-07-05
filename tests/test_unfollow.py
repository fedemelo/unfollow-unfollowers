from unfollow_unfollowers.unfollow import unfollow

from .factories import write_following, write_pending_follow_requests, write_username_list


def test_unfollow_removes_from_following(export_dir, tmp_path):
    write_following(export_dir, ["alice", "bob"])
    exclusions = tmp_path / "excluded.txt"
    known_disabled = tmp_path / "known_disabled.txt"

    changes = unfollow("alice", export_dir, exclusions, known_disabled)

    assert changes == ["following.json"]


def test_unfollow_removes_from_pending_requests(export_dir, tmp_path):
    write_pending_follow_requests(export_dir, ["dave"])
    exclusions = tmp_path / "excluded.txt"
    known_disabled = tmp_path / "known_disabled.txt"

    changes = unfollow("dave", export_dir, exclusions, known_disabled)

    assert changes == ["pending_follow_requests.json"]


def test_unfollow_removes_from_username_lists(export_dir, tmp_path):
    exclusions = tmp_path / "excluded.txt"
    known_disabled = tmp_path / "known_disabled.txt"
    write_username_list(exclusions, ["alice"])
    write_username_list(known_disabled, ["alice"])

    changes = unfollow("alice", export_dir, exclusions, known_disabled)

    assert changes == [str(exclusions), str(known_disabled)]


def test_unfollow_removes_from_everywhere_at_once(export_dir, tmp_path):
    write_following(export_dir, ["alice"])
    write_pending_follow_requests(export_dir, ["alice"])
    exclusions = tmp_path / "excluded.txt"
    known_disabled = tmp_path / "known_disabled.txt"
    write_username_list(exclusions, ["alice"])
    write_username_list(known_disabled, ["alice"])

    changes = unfollow("alice", export_dir, exclusions, known_disabled)

    assert changes == [
        "following.json",
        "pending_follow_requests.json",
        str(exclusions),
        str(known_disabled),
    ]


def test_unfollow_returns_empty_list_when_username_not_found(export_dir, tmp_path):
    write_following(export_dir, ["bob"])
    exclusions = tmp_path / "excluded.txt"
    known_disabled = tmp_path / "known_disabled.txt"

    assert unfollow("alice", export_dir, exclusions, known_disabled) == []

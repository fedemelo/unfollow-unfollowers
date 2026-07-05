import json

from unfollow_unfollowers.mutate import (
    remove_from_following,
    remove_from_pending_requests,
    remove_from_username_list,
)

from .factories import write_following, write_pending_follow_requests, write_username_list


def test_remove_from_following_removes_matching_entry(export_dir):
    write_following(export_dir, ["alice", "bob"])
    assert remove_from_following(export_dir, "alice") is True
    data = json.loads((export_dir / "following.json").read_text())
    usernames = {entry["title"] for entry in data["relationships_following"]}
    assert usernames == {"bob"}


def test_remove_from_following_returns_false_when_username_absent(export_dir):
    write_following(export_dir, ["bob"])
    assert remove_from_following(export_dir, "alice") is False


def test_remove_from_following_returns_false_when_file_missing(export_dir):
    assert remove_from_following(export_dir, "alice") is False


def test_remove_from_pending_requests_removes_matching_entry(export_dir):
    write_pending_follow_requests(export_dir, ["dave", "erin"])
    assert remove_from_pending_requests(export_dir, "dave") is True
    data = json.loads((export_dir / "pending_follow_requests.json").read_text())
    remaining = {
        lv["value"] for entry in data for lv in entry["label_values"] if lv["label"] == "Username"
    }
    assert remaining == {"erin"}


def test_remove_from_pending_requests_returns_false_when_username_absent(export_dir):
    write_pending_follow_requests(export_dir, ["erin"])
    assert remove_from_pending_requests(export_dir, "dave") is False


def test_remove_from_username_list_removes_matching_line(tmp_path):
    path = tmp_path / "list.txt"
    write_username_list(path, ["# comment", "alice", "bob"])
    assert remove_from_username_list(path, "alice") is True
    assert path.read_text().splitlines() == ["# comment", "bob"]


def test_remove_from_username_list_returns_false_when_username_absent(tmp_path):
    path = tmp_path / "list.txt"
    write_username_list(path, ["alice"])
    assert remove_from_username_list(path, "bob") is False


def test_remove_from_username_list_returns_false_when_file_missing(tmp_path):
    assert remove_from_username_list(tmp_path / "missing.txt", "alice") is False

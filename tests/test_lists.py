from unfollow_unfollowers.lists import apply_manual_filters, load_username_list

from .factories import write_username_list


def test_load_username_list_returns_empty_set_when_file_missing(tmp_path):
    assert load_username_list(tmp_path / "missing.txt") == set()


def test_load_username_list_skips_blank_lines_and_comments(tmp_path):
    path = tmp_path / "list.txt"
    write_username_list(path, ["# a comment", "", "alice", "  bob  ", "# another"])
    assert load_username_list(path) == {"alice", "bob"}


def test_apply_manual_filters_removes_deleted_accounts():
    usernames = {"alice", "__deleted__abc123"}
    assert apply_manual_filters(usernames) == {"alice"}


def test_apply_manual_filters_removes_exclusions_and_known_disabled():
    usernames = {"alice", "bob", "carol"}
    result = apply_manual_filters(usernames, exclusions={"bob"}, known_disabled={"carol"})
    assert result == {"alice"}

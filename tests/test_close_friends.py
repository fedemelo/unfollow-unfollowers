from unfollow_unfollowers.close_friends import diff_close_friends

from .factories import write_close_friends, write_username_list


def test_diff_close_friends_no_differences(export_dir, tmp_path):
    write_close_friends(export_dir, ["alice", "bob"])
    standard = tmp_path / "standard.txt"
    write_username_list(standard, ["alice", "bob"])

    unexpected, missing = diff_close_friends(export_dir, standard)

    assert unexpected == set()
    assert missing == set()


def test_diff_close_friends_reports_unexpected_additions(export_dir, tmp_path):
    write_close_friends(export_dir, ["alice", "bob"])
    standard = tmp_path / "standard.txt"
    write_username_list(standard, ["alice"])

    unexpected, missing = diff_close_friends(export_dir, standard)

    assert unexpected == {"bob"}
    assert missing == set()


def test_diff_close_friends_reports_missing_entries(export_dir, tmp_path):
    write_close_friends(export_dir, ["alice"])
    standard = tmp_path / "standard.txt"
    write_username_list(standard, ["alice", "carol"])

    unexpected, missing = diff_close_friends(export_dir, standard)

    assert unexpected == set()
    assert missing == {"carol"}


def test_diff_close_friends_missing_standard_file_treats_all_as_unexpected(export_dir, tmp_path):
    write_close_friends(export_dir, ["alice"])
    standard = tmp_path / "missing_standard.txt"

    unexpected, missing = diff_close_friends(export_dir, standard)

    assert unexpected == {"alice"}
    assert missing == set()

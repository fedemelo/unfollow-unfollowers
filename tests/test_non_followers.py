from unfollow_unfollowers.non_followers import find_non_followers

from .factories import write_followers, write_following


def test_find_non_followers_returns_following_minus_followers(export_dir):
    write_following(export_dir, ["alice", "bob", "carol"])
    write_followers(export_dir, ["alice"])
    assert find_non_followers(export_dir) == {"bob", "carol"}


def test_find_non_followers_excludes_deleted_accounts(export_dir):
    write_following(export_dir, ["bob", "__deleted__abc123"])
    write_followers(export_dir, [])
    assert find_non_followers(export_dir) == {"bob"}


def test_find_non_followers_applies_exclusions_and_known_disabled(export_dir):
    write_following(export_dir, ["bob", "carol", "dave"])
    write_followers(export_dir, [])
    result = find_non_followers(export_dir, exclusions={"bob"}, known_disabled={"carol"})
    assert result == {"dave"}

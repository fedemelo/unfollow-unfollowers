from unfollow_unfollowers.formatting import print_usernames, profile_url


def test_profile_url():
    assert profile_url("alice") == "https://www.instagram.com/alice"


def test_print_usernames_prints_count_and_sorted_urls(capsys):
    print_usernames({"Bob", "alice"}, "accounts")
    out = capsys.readouterr().out
    lines = out.splitlines()
    assert lines[0] == "2 accounts:"
    assert lines[1] == ""
    assert lines[2] == "https://www.instagram.com/alice"
    assert lines[3] == "https://www.instagram.com/Bob"


def test_print_usernames_handles_empty_iterable(capsys):
    print_usernames(set(), "accounts")
    out = capsys.readouterr().out
    assert out.splitlines()[0] == "0 accounts:"

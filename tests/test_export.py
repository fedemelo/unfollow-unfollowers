from unfollow_unfollowers.export import (
    entry_usernames,
    is_deleted_account,
    load_followers,
    load_following,
    load_pending_follow_requests,
    pending_request_username,
)

from .factories import write_followers, write_following, write_pending_follow_requests


def test_entry_usernames_reads_value_when_present():
    entry = {"string_list_data": [{"href": "https://www.instagram.com/foo", "value": "foo"}]}
    assert entry_usernames(entry) == {"foo"}


def test_entry_usernames_falls_back_to_href_when_value_missing():
    entry = {"string_list_data": [{"href": "https://www.instagram.com/_u/bar"}]}
    assert entry_usernames(entry) == {"bar"}


def test_entry_usernames_ignores_entries_without_string_list_data():
    assert entry_usernames({}) == set()


def test_load_followers_reads_single_file(export_dir):
    write_followers(export_dir, ["alice", "bob"])
    assert load_followers(export_dir) == {"alice", "bob"}


def test_load_followers_merges_paginated_files(export_dir):
    write_followers(export_dir, ["alice"], filename="followers_1.json")
    write_followers(export_dir, ["bob"], filename="followers_2.json")
    assert load_followers(export_dir) == {"alice", "bob"}


def test_load_following(export_dir):
    write_following(export_dir, ["alice", "carol"])
    assert load_following(export_dir) == {"alice", "carol"}


def test_pending_request_username_found():
    entry = {"label_values": [{"label": "Username", "value": "dave"}]}
    assert pending_request_username(entry) == "dave"


def test_pending_request_username_missing():
    entry = {"label_values": [{"label": "Name", "value": "Dave"}]}
    assert pending_request_username(entry) is None


def test_load_pending_follow_requests(export_dir):
    write_pending_follow_requests(export_dir, ["dave", "erin"])
    assert load_pending_follow_requests(export_dir) == {"dave", "erin"}


def test_is_deleted_account():
    assert is_deleted_account("__deleted__abc123")
    assert not is_deleted_account("regular_user")

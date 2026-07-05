.PHONY: install format lint test non-followers pending recheck-disabled unfollow close

install:
	python3 -m venv .venv
	.venv/bin/pip install -e ".[dev]"

format:
	.venv/bin/ruff format .
	.venv/bin/ruff check --fix .

lint:
	.venv/bin/ruff check .

test:
	.venv/bin/pytest

non-followers:
	python3 -m unfollow_unfollowers.non_followers

pending:
	python3 -m unfollow_unfollowers.pending_requests

recheck-disabled:
	python3 -m unfollow_unfollowers.non_followers --recheck-disabled

unfollow:
	python3 -m unfollow_unfollowers.unfollow $(username)

close:
	python3 -m unfollow_unfollowers.close_friends

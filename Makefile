.PHONY: build format lint test non-followers pending recheck-disabled unfollow close

build:
	cargo build

format:
	cargo fmt
	cargo clippy --all-targets --fix --allow-dirty

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings

test:
	cargo test

non-followers:
	cargo run --bin non_followers

pending:
	cargo run --bin pending_requests

recheck-disabled:
	cargo run --bin non_followers -- --recheck-disabled

unfollow:
	cargo run --bin unfollow -- $(username)

close:
	cargo run --bin close_friends

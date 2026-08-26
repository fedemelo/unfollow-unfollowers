# unfollow-unfollowers

Find Instagram accounts you follow that don't follow you back. Also find accounts
you've sent a follow request to that haven't answered. Both come from your own
data export — no bots, no API.

## Setup

1. Request your data export from Instagram (Settings → Accounts Center → Your
   information and permissions → Download your information). Choose **JSON** or
   **HTML** — the tool detects the format of each file on its own.
2. Unzip it into `export/` at the repo root, so the data ends up at
   `export/connections/followers_and_following/`. (You can unzip it elsewhere
   too — just pass that path explicitly.)
3. Install the [Rust toolchain](https://rustup.rs) if you don't already have it.
   `cargo build` fetches dependencies and compiles the binaries.

## Usage

```sh
cargo run --bin non_followers      # accounts you follow that don't follow you back
cargo run --bin pending_requests   # sent follow requests still pending
cargo run --bin close_friends      # diff your close friends list against a standard one
cargo run --bin unfollow -- <username>   # after you unfollow someone, so they stop showing up
```

Each read-only command also has a `make` shortcut (`make non-followers`, `make
pending`, `make close`, `make unfollow username=<username>`) and prints its
results to a file under `results/`.

Every command takes `--help` for its full set of options, including date
cutoffs and where results are read from and saved to.

## Development

```sh
cargo build    # compile
make format    # cargo fmt + clippy --fix
make lint      # cargo fmt --check + clippy
make test      # cargo test
```

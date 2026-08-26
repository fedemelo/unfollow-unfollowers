# unfollow-unfollowers

- Find Instagram accounts you follow that don't follow you back.
- Find accounts you've sent a follow request to that haven't answered.

## Setup

1. Request your data export from Instagram. [How to](./how-to-request-instagram-export.md).
2. Install the [Rust toolchain](https://rustup.rs) if you don't already have it.
   `cargo build` fetches dependencies and compiles the binaries.

## Usage

```sh
make non-followers      # accounts you follow that don't follow you back
make pending-requests   # sent follow requests still pending
make close-friends      # diff your close friends list against a standard one
make unfollow -- <username>   # after you unfollow someone, to delete it from the current export
```
Each command prints its results to a file under `results/`.

Every command takes `--help` for its full set of options, e.g., date cutoffs.

## Development

```sh
cargo build
make format
make lint
make test
```

# unfollow-unfollowers

Find Instagram accounts you follow that don't follow you back, and accounts you've
sent a follow request to that haven't responded — from your own data export
(no bots, no API).

## Setup

1. Request your data export from Instagram (Settings → Accounts Center → Your
   information and permissions → Download your information), in either **JSON**
   or **HTML** format — both are supported, auto-detected per file.
2. Unzip it into `export/` at the repo root, so the data lands at
   `export/connections/followers_and_following/` — the default `export_dir` every
   command uses. (Unzipping elsewhere works too; just pass that path explicitly.)
3. Install the [Rust toolchain](https://rustup.rs) if you don't already have it.
   `cargo build` will fetch dependencies and compile the binaries.

## Usage

```sh
cargo run --bin non_followers -- [export_dir]
cargo run --bin pending_requests -- [export_dir]
```

Or, with the default `export_dir`, via the Makefile shortcuts:

```sh
make non-followers
make pending
make recheck-disabled   # non-followers, but re-including known_disabled_accounts.txt
```

`export_dir` defaults to `export/connections/followers_and_following`. Each prints
the profile URLs of the matching accounts, and also saves the same report to a file
under `results/` (`results/non_followers.txt`, `results/pending_requests.txt`,
`results/close_friends.txt` — override with `--output <path>`). `results/` is
gitignored; each run overwrites the previous one, since the export is a static
snapshot anyway.

## Excluding recent accounts by date

`non_followers` and `pending_requests` both take `--exclude-after <YYYY-MM-DD>`,
to drop entries too recent to fairly judge yet:

```sh
cargo run --bin pending_requests -- --exclude-after 2026-08-20   # requests sent after Aug 20 excluded
cargo run --bin non_followers -- --exclude-after 2026-08-20      # accounts followed after Aug 20 excluded
```

For `pending_requests` this excludes requests sent after the date; for
`non_followers` it excludes accounts you started following after the date (they
may not have had time to follow back yet). `close_friends` has no date filter —
it's a diff against a hand-curated list, not something that needs "time to catch
up." An entry with no recorded date (can happen in older exports) is always kept,
since there's nothing to filter it by.

## Excluding accounts

Both commands share the same filters, backed by two hand-edited text files in `data/`
(one username per line, `#` for comments):

- `data/excluded_accounts.txt` — accounts to never suggest (e.g. a school page).
- `data/known_disabled_accounts.txt` — accounts you've manually confirmed are
  deleted/disabled. Filtered out by default so they don't keep reappearing.
- `--recheck-disabled` — temporarily ignore `known_disabled_accounts.txt` to
  re-verify by hand whether any of those accounts are active again.

Accounts already flagged as deleted in the export itself (`__deleted__...`) are
always filtered out automatically.

## After you unfollow someone

Your export is a static snapshot, so an account you just unfollowed will keep
showing up on every rerun until you re-export. Instead, tell the tool directly:

```sh
cargo run --bin unfollow -- <username>
# or
make unfollow username=<username>
```

This removes the username from `following.json` / `pending_follow_requests.json`
in your local export, and from `data/excluded_accounts.txt` /
`data/known_disabled_accounts.txt` if it was listed there — so it won't appear
again on the next run. `unfollow` only edits the JSON export; if yours is HTML,
re-export or edit the HTML file by hand.

## Close friends check

`data/standard_close_friends.txt` is your hand-curated "day ones" list — the
close friends you always expect to have active. Diff it against your actual
close friends list from the export:

```sh
cargo run --bin close_friends -- [export_dir]
# or
make close
```

Output is a diff: `-username` means they're in your actual close friends list
but not expected per your standard list; `+username` means they're expected per
your standard list but missing from the actual one.

## Project layout

```
src/
    export/
        mod.rs              # picks JSON vs. HTML per file, shared DatedUsername type
        json.rs             # parses the JSON export format
        html.rs             # parses the HTML export format
    mutate.rs               # removes a username from the local JSON export/lists after you unfollow
    lists.rs                # loads data/*.txt and applies exclusion/known-disabled filters
    formatting.rs           # profile URL formatting, report text, and saving to results/
    cli.rs                   # shared clap setup for the read-only commands
    non_followers.rs        # accounts you follow that don't follow back
    pending_requests.rs     # sent follow requests still pending
    unfollow.rs              # removes a username after you've unfollowed them
    close_friends.rs        # diffs actual vs. standard close friends list
    test_support.rs         # shared test fixtures (JSON/file builders)
    bin/                     # thin entry points, one per binary above
data/                      # your hand-curated username lists (gitignored)
results/                   # saved output from each run (gitignored)
```

## Development

```sh
cargo build    # compile
make format    # cargo fmt + clippy --fix
make lint      # cargo fmt --check + clippy
make test      # cargo test
```

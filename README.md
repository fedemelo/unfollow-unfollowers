# unfollow-unfollowers

Find Instagram accounts you follow that don't follow you back, and accounts you've
sent a follow request to that haven't responded — from your own data export
(no bots, no API).

## Setup

1. Request your data export from Instagram in **JSON** format (Settings → Accounts
   Center → Your information and permissions → Download your information).
2. Unzip it and note the path to `connections/followers_and_following/`.
3. Run `make install` to create a `.venv` and install the package with its dev
   dependencies (`ruff`, `pytest`).

## Usage

```sh
python3 -m unfollow_unfollowers.non_followers [export_dir]
python3 -m unfollow_unfollowers.pending_requests [export_dir]
```

Or, with the default `export_dir`, via the Makefile shortcuts:

```sh
make non-followers
make pending
make recheck-disabled   # non-followers, but re-including known_disabled_accounts.txt
```

`export_dir` defaults to `export/connections/followers_and_following`. Each prints
the profile URLs of the matching accounts.

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
python3 -m unfollow_unfollowers.unfollow <username>
# or
make unfollow username=<username>
```

This removes the username from `following.json` / `pending_follow_requests.json`
in your local export, and from `data/excluded_accounts.txt` /
`data/known_disabled_accounts.txt` if it was listed there — so it won't appear
again on the next run.

## Project layout

```
unfollow_unfollowers/
    export.py            # parses the raw Instagram export JSON
    mutate.py             # removes a username from the local export/lists after you unfollow
    lists.py              # loads data/*.txt and applies exclusion/known-disabled filters
    formatting.py         # profile URL formatting and output
    cli.py                 # shared argparse setup for the read-only commands
    non_followers.py      # accounts you follow that don't follow back
    pending_requests.py   # sent follow requests still pending
    unfollow.py            # removes a username after you've unfollowed them
tests/                    # unit tests, one file per module above
data/                     # your hand-curated username lists (gitignored)
```

## Development

```sh
make install   # create .venv and install with dev dependencies
make format    # ruff format + autofix
make lint      # ruff check
make test      # pytest
```

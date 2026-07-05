# unfollow-unfollowers

Find Instagram accounts you follow that don't follow you back — from your own
data export (no bots, no API).

## Setup

1. Request your data export from Instagram in **JSON** format (Settings → Accounts
   Center → Your information and permissions → Download your information).
2. Unzip it and note the path to `connections/followers_and_following/`.

## Usage

```sh
python3 -m unfollow_unfollowers.non_followers [export_dir]
```

Or, with the default `export_dir`, via the Makefile shortcuts:

```sh
make non-followers
make recheck-disabled   # non-followers, but re-including known_disabled_accounts.txt
```

`export_dir` defaults to `export/connections/followers_and_following`. Prints
the profile URLs of the matching accounts.

## Excluding accounts

Backed by two hand-edited text files in `data/` (one username per line, `#` for
comments):

- `data/excluded_accounts.txt` — accounts to never suggest (e.g. a school page).
- `data/known_disabled_accounts.txt` — accounts you've manually confirmed are
  deleted/disabled. Filtered out by default so they don't keep reappearing.
- `--recheck-disabled` — temporarily ignore `known_disabled_accounts.txt` to
  re-verify by hand whether any of those accounts are active again.

Accounts already flagged as deleted in the export itself (`__deleted__...`) are
always filtered out automatically.

## Project layout

```
unfollow_unfollowers/
    export.py         # parses the raw Instagram export JSON
    lists.py           # loads data/*.txt and applies exclusion/known-disabled filters
    formatting.py      # profile URL formatting and output
    cli.py              # shared argparse setup
    non_followers.py   # accounts you follow that don't follow back
data/                  # your hand-curated username lists (gitignored)
```

---
name: refresh-reports
description: Locates a freshly downloaded Instagram data export zip, unzips it into this repo's export/ directory (clearing out whatever was there before), and runs the non-followers, pending-requests, and close-friends reports. Use when asked to process, import, or refresh the Instagram export, or to regenerate the reports from a new download.
---

When asked to process a new Instagram export and regenerate the reports, follow
these steps in order.

## 1. Find the zip

If the skill was invoked with an explicit path to a zip (e.g. via `ARGUMENTS`),
that path **is** the new export — use it directly and skip the search below.
The path a user gives you always names the new export, never something to be
cleaned away.

Otherwise, run the search script — it deterministically checks, in fixed
order, this repo's `export/` folder, then `~/Desktop`, then `~/Downloads`,
short-circuiting on the first location whose newest `instagram-*.zip` was
modified today, and otherwise falling back to the overall newest match:

```sh
.claude/skills/refresh-reports/scripts/find_export.sh
```

It prints the chosen zip's path on success. If it exits non-zero (no output,
no match anywhere), ask the user where to look — they'll typically name a
folder like "check the Desktop" — map that to the actual path and check there
directly. Don't guess a location the user didn't name and that isn't one of
the three the script already covers.

## 2. Clear out the old export

Before bringing in the new one, stale data from a previous run must not linger
alongside it. `export/` is gitignored, so all of this is local cleanup only,
not a git operation.

**Check whether the found zip already lives inside `export/` first** — resolve
its path and compare against the repo's `export/` directory:

- **If it's already in `export/`** (this happens when the user hands you a
  path there directly, as in step 1): do **not** run `make clean` — that
  target deletes `export/*.zip` and would destroy the very file you're about
  to process. Instead, only remove the stale unzipped folder and any *other*
  zips sitting next to it:
  ```sh
  rm -rf export/connections
  find export -maxdepth 1 -name '*.zip' ! -name "$(basename <found_zip>)" -delete
  ```
- **If it's anywhere else** (e.g. `~/Downloads`): it's safe to fully clean
  first, since the source file is untouched by this:
  ```sh
  make clean
  ```

## 3. Move and unzip

If the zip isn't already in `export/`, move it there; then unzip it (unzipping
is safe either way — it just overwrites the same-named folder):

```sh
mv <found_zip> export/   # skip this if <found_zip> is already inside export/
unzip -q "export/$(basename <found_zip>)" -d export/
```

Confirm `export/connections/followers_and_following/` exists afterward — that's
what every report command reads from.

## 4. Run the reports

Run all three read-only commands with their defaults — **do not ask the user
for a cutoff date, and don't pass `--exclude-after` unless they explicitly
asked for one in this request**:

```sh
cargo run --bin non_followers
cargo run --bin pending_requests
cargo run --bin close_friends
```

Let each command run to completion rather than piping its output through
`head`/`tail`/etc. — each one writes its report file only after finishing its
stdout output, and cutting the pipe early can kill the process with SIGPIPE
before it saves the file.

If the user did ask for a cutoff date in this request, pass
`--exclude-after <YYYY-MM-DD>` to `non_followers` and/or `pending_requests` as
they specified — `close_friends` has no such flag.

## 5. Report back

Once all three commands have saved their files, respond with nothing but these
three lines (paths relative to the repo root, matching each command's
`--output` default unless the user overrode it):

```
Non-followers: results/non_followers.txt
Pending requests: results/pending_requests.txt
Close friends: results/close_friends.txt
```

No counts, no preamble, no extra commentary — just the three lines.

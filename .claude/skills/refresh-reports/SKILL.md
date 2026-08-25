---
name: refresh-reports
description: Locates a freshly downloaded Instagram data export zip, unzips it into this repo's export/ directory (clearing out whatever was there before), and runs the non-followers, pending-requests, and close-friends reports. Use when asked to process, import, or refresh the Instagram export, or to regenerate the reports from a new download.
---

When asked to process a new Instagram export and regenerate the reports, follow
these steps in order.

## 1. Find the zip

Instagram names the export zip `instagram-<username>-<date>-<random>.zip` (e.g.
`instagram-federico.melo-2026-08-25-drOUtrtK.zip`).

Search `~/Downloads` first — that's where a browser drops a fresh download by
default:

```sh
ls -t ~/Downloads/instagram-*.zip 2>/dev/null | head -1
```

If nothing matches there, ask the user where to look. They'll typically say
something like "check the Desktop" or "it's in Downloads" — map that to the
actual folder (`~/Desktop`, `~/Downloads`, etc.) and repeat the search there.
Don't guess a location the user didn't name and that isn't the `~/Downloads`
default.

If more than one match turns up in the same folder, use the most recently
modified one (`ls -t` already sorts newest first) — that's the freshest export.

## 2. Clear out the old export

Before bringing in the new one, remove whatever is currently in this repo's
`export/` directory — the previous zip and the previously unzipped
`connections/` folder — so stale data never lingers alongside the new export:

```sh
rm -rf export/connections export/*.zip
```

`export/` is gitignored, so this is local cleanup only, not a git operation.

## 3. Move and unzip

Move the zip found in step 1 into `export/`, then unzip it there:

```sh
mv <found_zip> export/
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

#!/usr/bin/env bash
# Deterministically locates the freshest Instagram export zip.
#
# Search order (fixed): this repo's export/ folder, ~/Desktop, ~/Downloads.
# Within each location, the newest instagram-*.zip by mtime is considered.
# Short-circuits (prints and exits) as soon as a location's newest match was
# modified today. If no location has a today match, falls back to the overall
# newest match across all three. Prints nothing and exits 1 if none found.
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/../../../.." && pwd)"

locations=(
  "$repo_root/export"
  "$HOME/Desktop"
  "$HOME/Downloads"
)

today="$(date +%Y-%m-%d)"
best_path=""
best_mtime=0

for dir in "${locations[@]}"; do
  [ -d "$dir" ] || continue

  newest_path=""
  newest_mtime=0
  while IFS= read -r -d '' f; do
    mtime="$(stat -f %m "$f")"
    if [ "$mtime" -gt "$newest_mtime" ]; then
      newest_mtime="$mtime"
      newest_path="$f"
    fi
  done < <(find "$dir" -maxdepth 1 -iname 'instagram-*.zip' -print0 2>/dev/null)

  [ -z "$newest_path" ] && continue

  file_date="$(date -r "$newest_mtime" +%Y-%m-%d)"
  if [ "$file_date" = "$today" ]; then
    echo "$newest_path"
    exit 0
  fi

  if [ "$newest_mtime" -gt "$best_mtime" ]; then
    best_mtime="$newest_mtime"
    best_path="$newest_path"
  fi
done

if [ -n "$best_path" ]; then
  echo "$best_path"
  exit 0
fi

exit 1

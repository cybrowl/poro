#!/usr/bin/env bash

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

staged_files="$(git diff --cached --name-only --diff-filter=ACMR)"

if [[ -z "${staged_files}" ]]; then
  exit 0
fi

blocked_path_patterns=(
  '^\.harness/'
  '(^|/)target/'
  '^tauri/target/'
  '(^|/)harness-server(\.exe)?$'
)

blocked_content_patterns=(
  'XAI_API_KEY\s*='
  'export\s+XAI_API_KEY'
  'xai-[A-Za-z0-9]{24,}'
)

violations=()

while IFS= read -r path; do
  [[ -z "$path" ]] && continue

  for pattern in "${blocked_path_patterns[@]}"; do
    if [[ "$path" =~ $pattern ]]; then
      violations+=("Blocked staged path: $path")
      continue 2
    fi
  done

  if ! content="$(git show ":$path" 2>/dev/null)"; then
    continue
  fi

  for pattern in "${blocked_content_patterns[@]}"; do
    if printf '%s' "$content" | perl -0ne "exit((m{$pattern}m) ? 0 : 1)"; then
      violations+=("Blocked staged content in $path matching /$pattern/")
      continue 2
    fi
  done
done <<< "$staged_files"

if (( ${#violations[@]} > 0 )); then
  printf 'Private asset guard failed:\n' >&2
  printf ' - %s\n' "${violations[@]}" >&2
  printf '\n' >&2
  printf 'Unstage or redact the flagged content before committing.\n' >&2
  exit 1
fi

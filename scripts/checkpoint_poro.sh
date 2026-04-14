#!/usr/bin/env bash
set -euo pipefail

SOURCE_DIR="${SOURCE_DIR:-/Users/cyberowl/Repos/poro}"
PARENT_DIR="${PARENT_DIR:-$(cd "${SOURCE_DIR}/.." && pwd)}"
CHECKPOINT_DIR="${CHECKPOINT_DIR:-${PARENT_DIR}/poro_checkpoint}"
ARCHIVE_DIR="${ARCHIVE_DIR:-${CHECKPOINT_DIR}/archives}"
MANIFEST_DIR="${MANIFEST_DIR:-${CHECKPOINT_DIR}/manifests}"
LOG_DIR="${LOG_DIR:-${CHECKPOINT_DIR}/logs}"

mkdir -p "${ARCHIVE_DIR}" "${MANIFEST_DIR}" "${LOG_DIR}"

if [[ ! -d "${SOURCE_DIR}" ]]; then
  echo "Missing source dir: ${SOURCE_DIR}" >&2
  exit 1
fi

timestamp_local="$(date +"%Y%m%d-%H%M%S")"
timestamp_utc="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

git_branch="detached"
git_sha="no-git"
git_dirty="unknown"

if git -C "${SOURCE_DIR}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  git_branch="$(git -C "${SOURCE_DIR}" rev-parse --abbrev-ref HEAD 2>/dev/null || echo detached)"
  git_sha="$(git -C "${SOURCE_DIR}" rev-parse --short HEAD 2>/dev/null || echo unknown)"
  if [[ -n "$(git -C "${SOURCE_DIR}" status --short 2>/dev/null || true)" ]]; then
    git_dirty="true"
  else
    git_dirty="false"
  fi
fi

safe_branch="$(printf '%s' "${git_branch}" | tr '/[:space:]' '--' | tr -cd '[:alnum:]._-' )"
if [[ -z "${safe_branch}" ]]; then
  safe_branch="detached"
fi

archive_base="poro_${timestamp_local}__${safe_branch}__${git_sha}"
archive_path="${ARCHIVE_DIR}/${archive_base}.tar.gz"
manifest_path="${MANIFEST_DIR}/${archive_base}.json"

tar \
  --exclude='poro/node_modules' \
  --exclude='poro/build' \
  --exclude='poro/.svelte-kit' \
  --exclude='poro/.dfx' \
  --exclude='poro/.harness' \
  --exclude='poro/tauri/target' \
  --exclude='poro/tauri/gen/schemas' \
  --exclude='poro/.DS_Store' \
  -czf "${archive_path}" \
  -C "${PARENT_DIR}" \
  poro

sha256="$(shasum -a 256 "${archive_path}" | awk '{print $1}')"
size_bytes="$(stat -f%z "${archive_path}")"

cat > "${manifest_path}" <<EOF
{
  "created_at_utc": "${timestamp_utc}",
  "source_dir": "${SOURCE_DIR}",
  "checkpoint_dir": "${CHECKPOINT_DIR}",
  "archive_path": "${archive_path}",
  "sha256": "${sha256}",
  "size_bytes": ${size_bytes},
  "git_branch": "${git_branch}",
  "git_sha": "${git_sha}",
  "git_dirty": ${git_dirty},
  "excluded_paths": [
    "poro/node_modules",
    "poro/build",
    "poro/.svelte-kit",
    "poro/.dfx",
    "poro/.harness",
    "poro/tauri/target",
    "poro/tauri/gen/schemas",
    "poro/.DS_Store"
  ]
}
EOF

ln -sfn "archives/${archive_base}.tar.gz" "${CHECKPOINT_DIR}/latest.tar.gz"
ln -sfn "manifests/${archive_base}.json" "${CHECKPOINT_DIR}/latest.json"

printf '%s\n' "${archive_path}"

#!/usr/bin/env bash
set -euo pipefail

LABEL="${LABEL:-com.cybrowl.poro.checkpoint}"
START_INTERVAL_SECS="${START_INTERVAL_SECS:-21600}"
SOURCE_DIR="${SOURCE_DIR:-/Users/cyberowl/Repos/poro}"
PARENT_DIR="${PARENT_DIR:-$(cd "${SOURCE_DIR}/.." && pwd)}"
CHECKPOINT_DIR="${CHECKPOINT_DIR:-${PARENT_DIR}/poro_checkpoint}"
ARCHIVE_DIR="${ARCHIVE_DIR:-${CHECKPOINT_DIR}/archives}"
MANIFEST_DIR="${MANIFEST_DIR:-${CHECKPOINT_DIR}/manifests}"
LOG_DIR="${LOG_DIR:-${CHECKPOINT_DIR}/logs}"
SCRIPT_PATH="${SCRIPT_PATH:-${SOURCE_DIR}/scripts/checkpoint_poro.sh}"
PLIST_PATH="${PLIST_PATH:-${HOME}/Library/LaunchAgents/${LABEL}.plist}"

mkdir -p "${ARCHIVE_DIR}" "${MANIFEST_DIR}" "${LOG_DIR}" "$(dirname "${PLIST_PATH}")"

if [[ ! -x "${SCRIPT_PATH}" ]]; then
  echo "Checkpoint script must exist and be executable: ${SCRIPT_PATH}" >&2
  exit 1
fi

cat > "${PLIST_PATH}" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>${LABEL}</string>

  <key>ProgramArguments</key>
  <array>
    <string>${SCRIPT_PATH}</string>
  </array>

  <key>EnvironmentVariables</key>
  <dict>
    <key>SOURCE_DIR</key>
    <string>${SOURCE_DIR}</string>
    <key>PARENT_DIR</key>
    <string>${PARENT_DIR}</string>
    <key>CHECKPOINT_DIR</key>
    <string>${CHECKPOINT_DIR}</string>
    <key>ARCHIVE_DIR</key>
    <string>${ARCHIVE_DIR}</string>
    <key>MANIFEST_DIR</key>
    <string>${MANIFEST_DIR}</string>
    <key>LOG_DIR</key>
    <string>${LOG_DIR}</string>
    <key>PATH</key>
    <string>/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
  </dict>

  <key>RunAtLoad</key>
  <true/>

  <key>StartInterval</key>
  <integer>${START_INTERVAL_SECS}</integer>

  <key>WorkingDirectory</key>
  <string>${SOURCE_DIR}</string>

  <key>StandardOutPath</key>
  <string>${LOG_DIR}/launchd.stdout.log</string>

  <key>StandardErrorPath</key>
  <string>${LOG_DIR}/launchd.stderr.log</string>
</dict>
</plist>
EOF

launchctl bootout "gui/$(id -u)" "${PLIST_PATH}" >/dev/null 2>&1 || true
launchctl bootstrap "gui/$(id -u)" "${PLIST_PATH}"
launchctl kickstart -k "gui/$(id -u)/${LABEL}"

echo "${PLIST_PATH}"

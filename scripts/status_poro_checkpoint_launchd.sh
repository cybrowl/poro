#!/usr/bin/env bash
set -euo pipefail

LABEL="${LABEL:-com.cybrowl.poro.checkpoint}"
PLIST_PATH="${PLIST_PATH:-${HOME}/Library/LaunchAgents/${LABEL}.plist}"

echo "plist=${PLIST_PATH}"

if [[ -f "${PLIST_PATH}" ]]; then
  echo "plist_exists=true"
else
  echo "plist_exists=false"
fi

launchctl print "gui/$(id -u)/${LABEL}"

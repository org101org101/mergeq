#!/usr/bin/env bash

echo "BUILDKITE_BRANCH: $BUILDKITE_BRANCH"

cat <<EOF | tee /dev/tty | buildkite-agent pipeline upload
steps:
  - name: "main"
    command: "./check.sh"
    timeout_in_minutes: 20
    agents:
      queue: "default"
EOF

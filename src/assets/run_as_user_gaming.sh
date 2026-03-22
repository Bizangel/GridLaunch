#!/bin/bash

set -e

# Configuration
PULSE_PORT="4713"

# Check if user and application were specified
if [ $# -lt 2 ]; then
    echo "Error: Missing arguments"
    echo "Usage: run_as_user_gaming.sh <user> <application> [arguments...]"
    echo "Example: run_as_user_gaming.sh game-user steam"
    echo "Example: run_as_user_gaming.sh game-user firefox https://example.com"
    exit 1
fi

# First argument = isolated user
ISOLATED_USER="$1"
shift   # remove user from argument list so $@ becomes the application

# Validate that user exists
if ! id "${ISOLATED_USER}" &>/dev/null; then
    echo "Error: User '${ISOLATED_USER}' does not exist"
    exit 1
fi

xhost "+SI:localuser:${ISOLATED_USER}"
# Only load PulseAudio module if not already loaded
if ! pactl list modules short | grep -q "module-native-protocol-tcp.*127.0.0.1"; then
    pactl load-module module-native-protocol-tcp auth-ip-acl=127.0.0.1 auth-anonymous=0 port=${PULSE_PORT}
fi

# Launch your application
exec sudo -u ${ISOLATED_USER} DISPLAY=$DISPLAY ENABLE_GAMESCOPE_WSI=$ENABLE_GAMESCOPE_WSI PULSE_SERVER=tcp:127.0.0.1:${PULSE_PORT} dbus-run-session "$@"

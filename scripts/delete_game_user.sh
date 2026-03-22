#!/bin/bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <username>"
  exit 1
fi

CURRENT_USER="$(whoami)"
NEW_USER="game-user-$1"
SUDOERS_RULE_PATH="/etc/sudoers.d/$NEW_USER-nopasswd-as-${CURRENT_USER}"

if [[ ! "$1" =~ ^[a-zA-Z0-9_-]+$ ]]; then
  echo "Error: username suffix must be alphanumeric (hyphens/underscores allowed)"
  exit 1
fi

if ! id "$NEW_USER" &>/dev/null; then
    echo "User $NEW_USER does not exist"
    exit 1
fi

cat <<EOF
This script will require sudo elevated privileges.

This script will delete an existing game-user "$NEW_USER".

- It WILL delete the user's home directory.
- It will delete the existing user's sudoers file under $SUDOERS_RULE_PATH
EOF

read -p "Continue? (y/n): " answer
case "$answer" in
  [yY]) echo "Continuing..." ;;
  [nN]) echo "Aborted."; exit 1 ;;
  *) echo "Please answer y or n."; exit 1 ;;
esac

read -p "Are you absolutely sure? (y/n): " answer
case "$answer" in
  [yY]) echo "Continuing..." ;;
  [nN]) echo "Aborted."; exit 1 ;;
  *) echo "Please answer y or n."; exit 1 ;;
esac

echo "Deleting $SUDOERS_RULE_PATH"
sudo rm -f "$SUDOERS_RULE_PATH"
sudo userdel -r $NEW_USER
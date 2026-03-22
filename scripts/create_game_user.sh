#!/bin/bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <username>"
  exit 1
fi

CURRENT_USER="$(whoami)"
NEW_USER="game-user-$1"
SUDOERS_RULE_PATH="/etc/sudoers.d/$NEW_USER-nopasswd-as-${CURRENT_USER}"
NO_LOGIN_SHELL=$(which nologin)

if [[ ! "$1" =~ ^[a-zA-Z0-9_-]+$ ]]; then
  echo "Error: username suffix must be alphanumeric (hyphens/underscores allowed)"
  exit 1
fi

if ! getent group gamepad > /dev/null; then
  echo "Error: 'gamepad' group does not exist"
  exit 1
fi

if id "$NEW_USER" &>/dev/null; then
    echo "User $NEW_USER already exists"
    exit 1
fi

cat <<EOF
This script will require sudo elevated privileges.

This script will create a new user "$NEW_USER".

- It will create the new user's home under regular /home/$NEW_USER directory.
- The user's shell will be set to $NO_LOGIN_SHELL
- The user's password will be locked.
- It will add this new user to the "gamepad" group.
- A new SUDOERS rule will be created under $SUDOERS_RULE_PATH to allow current $USER (current user) to execute commands as said user without sudo prompts.
EOF

read -p "Continue? (y/n): " answer
case "$answer" in
  [yY]) echo "Continuing..." ;;
  [nN]) echo "Aborted."; exit 1 ;;
  *) echo "Please answer y or n."; exit 1 ;;
esac

echo "Creating game user"
sudo useradd -m "$NEW_USER"
echo "Locking shell"
sudo usermod -s "$NO_LOGIN_SHELL" "$NEW_USER"
sudo passwd -l "$NEW_USER"

echo "Adding user to gamepad group"
sudo usermod -aG gamepad "$NEW_USER"

echo "Adding SUDOERS rule"
RULE="${CURRENT_USER} ALL=(${NEW_USER}) NOPASSWD: ALL"
echo "$RULE" | sudo tee "$SUDOERS_RULE_PATH" > /dev/null
sudo chmod 0440 "$SUDOERS_RULE_PATH"
sudo visudo -cf "$SUDOERS_RULE_PATH" && echo "Rule safely installed in $SUDOERS_RULE_PATH"

echo "Done"
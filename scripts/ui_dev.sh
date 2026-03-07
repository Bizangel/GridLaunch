#!/usr/bin/env bash
# Exit immediately if a command exits with a non-zero status
set -e
cd "$(dirname "$0")"
cd ..

# Trap Ctrl+C (SIGINT) and kill all background jobs
trap 'echo "Stopping..."; kill 0' SIGINT

# Start Vite (frontend) in background
echo "Starting Vite..."
cd src-ui
npm run dev &

cd ..
# Start Rust (backend/UI)
echo "Starting Rust app..."
cargo run -- ui &

# Wait for both to exit
wait
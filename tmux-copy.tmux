#!/usr/bin/env bash

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}" )" && pwd)"
tmux bind-key "f" run-shell -b "cd $CURRENT_DIR && tmux new-window -d target/release/tmux-copy"

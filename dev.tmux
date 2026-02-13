#!/usr/bin/env bash

SESSION="dev"

tmux new-session -d -s $SESSION \
  'deno run --allow-net --allow-read --watch=main.ts,static/output.css,pkg/ main.ts'

tmux split-window -h -t $SESSION \
  'tailwindcss -i ./static/input.css -o ./static/output.css --watch'

tmux split-window -v -t $SESSION:0.1 \
  'cargo watch -i .gitignore -s "wasm-pack build --target web"'

tmux select-layout -t $SESSION tiled
tmux attach -t $SESSION

dev:
  nix develop

serve:
 deno run main.ts 

fmt:
  deno fmt tailwind.config.js main.ts && cargo fmt --all -v

build_wasm:
  wasm-pack build --target web --release

compile_css:
  tailwindcss -i ./static/input.css -o ./static/output.css --minify

ship:
   git add -A && git commit -m "migration" && git push

dev_serve:
   deno run --allow-net --allow-read --watch=main.ts,static/output.css,pkg/ main.ts

watch_css:
    tailwindcss -i ./static/input.css -o ./static/output.css --watch

watch_wasm:
    cargo watch -i .gitignore -s "wasm-pack build --target web"

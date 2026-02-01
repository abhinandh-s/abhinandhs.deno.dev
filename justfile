dev:
  nix develop

serve:
 deno run main.ts 

ship:
   git add -A && git commit -m "migration" && git push

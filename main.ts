import { serve } from "https://deno.land/std@0.224.0/http/server.ts";
import init, { render } from "./pkg/yew_blog.js";

console.log("Server running on http://localhost:8000");

await init();

serve(async (req) => {
  // Call the Rust function!
  const appHtml = await render();

  const html = `
    <!DOCTYPE html>
    <html>
      <head>
        <title>Abhinandh S</title>
      </head>
      <body>
        <div id="app">${appHtml}</div>
      </body>
    </html>
  `;

  return new Response(html, {
    headers: { "content-type": "text/html; charset=utf-8" },
  });
});

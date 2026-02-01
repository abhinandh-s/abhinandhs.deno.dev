import { serve } from "https://deno.land/std@0.224.0/http/server.ts";
import init, { render } from "./pkg/yew_blog.js";

const wasmUrl = new URL("./pkg/yew_blog_bg.wasm", import.meta.url);
await init(wasmUrl);

console.log("Server started running...");

serve(async (req) => {
  const url = new URL(req.url);
  try {
    const appHtml = await render(url.pathname);

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
  } catch (err) {
    console.error("SSR Rendering Error:", err);
    return new Response("Internal Server Error", { status: 500 });
    
  }
});

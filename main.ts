import { serve } from "https://deno.land/std@0.224.0/http/server.ts";
import { serveDir } from "https://deno.land/std@0.224.0/http/file_server.ts";
import init, { render } from "./pkg/yew_blog.js";

// Initialize Wasm
const wasmUrl = new URL("./pkg/yew_blog_bg.wasm", import.meta.url);
await init(wasmUrl);

console.log("Server started running...");

serve(async (req) => {
  const url = new URL(req.url);

  // Serve static assets (CSS, favicons, json feeds)
  if (
    url.pathname.startsWith("/static/") ||
    url.pathname.startsWith("/pkg/") ||
    url.pathname.includes("favicon") ||
    url.pathname.endsWith(".json") ||
    url.pathname.endsWith(".xml")
  ) {
    return serveDir(req, { fsRoot: "." });
  }

  try {
    const appHtml = await render(url.pathname);

    const html = `
 

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="Portfolio and articles by Abhinandh S - Software Engineer and Rust enthusiast.">
    <title>Abhinandh S</title>
    
    <link rel="stylesheet" href="/static/output.css"/>
  
    <link rel="preload" href="/pkg/yew_blog_bg.wasm" as="fetch" type="application/wasm" crossorigin="">

    <script type="module">
      import init from "/pkg/yew_blog.js";
      init("/pkg/yew_blog_bg.wasm");
    </script>

    <link rel="icon" href="/static/favicon/favicon.png" type="image/png" />
    <link rel="icon" href="/static/favicon/favicon.svg" type="image/svg+xml" />
    <link rel="icon" href="/static/favicon/favicon.ico" type="image/x-icon" />
    
    <link rel="alternate" type="application/feed+json" title="JSON Feed" href="/feed.json"/>
    <link rel="alternate" type="application/rss+xml" title="RSS" href="/feed.xml" />
    <link rel="alternate" type="application/atom+xml" title="Atom" href="/feed.atom.xml" />
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

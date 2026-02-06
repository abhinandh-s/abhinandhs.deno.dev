---
title: Yew.rs and SSR: The Full-Circle Migration
published_at: 2026-02-06
updated_at: 2026-02-06
snippet: How I finally achieved the "Rust Dream" by migrating from Deno Fresh to Yew.rs with SSR.
---

Years ago, when i dicided to build my own website, first thing i looked for was
how to build it in **rust**. Since rust is my first language, i felt I had the upper hand there.

**My requirements were**:

- Built in rust 
- Free deployment
- High performance
- Stylable
- Serverless 

Well that was too much to ask for.

The only option for rust was to build a website
using web assembly. I managed to get a site up and running on Github Pages.
But the load time was painful.

Eventually, I compromised. I learned TypeScript and rebuilt the site using the **Deno Fresh** framework. 

It served me for like 2 years.

### The 2026 Breakthrough

This year, I revisited my original dream. But this time, I finally managed to "duct-tape" all my requirements together into a high-performance stack. All because Deno now supports web assembly.

- In rust => yew.rs 
- Free deploy => Deno deploy
- High performance => Server Side Rendering
- Styling => tailwindcss

* **Live Site:** [abhinandhs.deno.dev](https://abhinandhs.deno.dev)
* **Source Code:** [GitHub Repository](https://github.com/abhinandh-s/abhinandhs.deno.dev)

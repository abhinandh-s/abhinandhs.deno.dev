---
title: Yew.rs and SSR - The Circle of Migration
published_at: 2026-02-06
updated_at: 2026-02-06
snippet: How I finally achieved the "Rust Dream" by migrating from Deno Fresh to Yew.rs with SSR.
tags: ["rust", "blog"]
---

Years ago, when i dicided to build my own website, first thing i looked for was
how to build it in **rust**. Since rust is my first language, i felt I had the upper hand there.


**My requirements were**:


- [ ] Built in rust 
- [ ] Free deployment
- [ ] High performance
- [ ] Stylable
- [ ] Serverless 


Well that was too much to ask for.


The only option for rust was to build a website
using web assembly. I took a look at many frameworks and finally settled on **yew.rs**.
I managed to get a site up and running on Github Pages.
But the load time was painful.


Eventually, I compromised. I learned TypeScript and rebuilt the site using the **Deno Fresh** framework. 


It served me for like 2 years.


### The 2026 Breakthrough


This year, I revisited my original dream. But this time, 
I finally managed to "duct-tape" all my requirements together into a high-performance stack.
All because Deno now supports web assembly. Now i am back on yew.rs again.

- [x] Built in rust  => yew.rs
- [x] Free deployment  => Deno deploy
- [x] High performance  => Server Side Rendering
- [x] Stylable  => tailwindcss
- [x] Serverless

* **Live Site:** [abhinandhs.deno.dev](https://abhinandhs.deno.dev)
* **Source Code:** [GitHub Repository](https://github.com/abhinandh-s/abhinandhs.deno.dev)

I have put a close to bare minimum yew on deno setup - 

* **Live Site:** [abhi.deno.dev](https://abhi.deno.dev)
* **Source Code:** [GitHub Repository](https://github.com/abhinandh-s/yew-deno)

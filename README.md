# Forest Explorer

Forest Explorer is a server-less inspector of the Filecoin blockchain.

# Implementation

[Rust](https://www.rust-lang.org/)+[Leptos](https://leptos.dev/) application which is compiled to a server [WASM](https://webassembly.org/) module and a client WASM module. The server module is hosted by [CloudFlare](https://workers.cloudflare.com/). It pre-renders a HTML response and [hydrates](https://book.leptos.dev/ssr/index.html) it (ie. add reactivity) via the client WASM module.

Anything pushed to `main` will automatically be deployed at <https://forest-explorer.chainsafe.dev>.

Running `npx wrangler@latest dev` will spawn a local copy of the explorer.

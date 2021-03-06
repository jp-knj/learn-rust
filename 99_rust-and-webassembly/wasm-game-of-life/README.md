<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with ð¦ð¸ by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## Directory
```shell
wasm-game-of-life/
âââ Cargo.toml    // ã¡ã¿ãã¼ã¿ãæå®(Rustã®ããã±ã¼ã¸ããã¼ã¸ã£ã§ãã«ããã¼ã«)
âââ LICENSE_APACHE
âââ LICENSE_MIT
âââ README.md
âââ src
    âââ lib.rs    // WebAssemblyã«ã³ã³ãã¤ã«ããRustã¯ã¬ã¼ãã®ã«ã¼ã
    âââ utils.rs  // Rustãä½¿ã£ã¦ä½æ¥­ãããã¨ãç°¡åã«ãã
```

`pkg`ãã£ã¬ã¯ããª
```shell
pkg/
âââ package.json
âââ README.md
âââ wasm_game_of_life_bg.wasm
âââ wasm_game_of_life.d.ts
âââ wasm_game_of_life.js
```
`.wasm`ãã¡ã¤ã«ã¯Rustã®ã½ã¼ã¹ããRustã³ã³ãã¤ã©ã«ãã£ã¦çæãããWebAssemblyã®ãã¤ããªã
Rustã®é¢æ°ã¨ãã¼ã¿å¨ã¦ã®Wasmã«ã³ã³ãã¤ã«ããããã¼ã¸ã§ã³ãå«ãã

`wasm-game-of-life/www`ãã£ã¬ã¯ããª
```shell
wasm-game-of-life/www/
âââ bootstrap.js        
âââ index.html          // ã«ã¼ãHTMLãã¡ã¤ã«
âââ index.js            // JavaScriptãã¡ã¤ã«ã®ã¨ã³ããªã¼ãã¤ã³ã
âââ LICENSE-APACHE
âââ LICENSE-MIT
âââ package.json        // wasm-pack-template ããã±ã¼ã¸ã®ãã¼ã¸ã§ã³ã§ãã hello-wasm-pack ã®ä¾å­é¢ä¿ãäºåã«è¨­å® 
âââ README.md
âââ webpack.config.js   // webpackãç°å¢è¨­å®
```
## About

[**ð Read this template tutorial! ð**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## ð´ Usage

### ð Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### ð ï¸ Build with `wasm-pack build`

```
wasm-pack build
```

### ð¬ Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### ð Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## ð Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

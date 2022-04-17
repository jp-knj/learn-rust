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

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## Directory
```shell
wasm-game-of-life/
├── Cargo.toml    // メタデータを指定(Rustのパッケージマネージャでビルドツール)
├── LICENSE_APACHE
├── LICENSE_MIT
├── README.md
└── src
    ├── lib.rs    // WebAssemblyにコンパイルするRustクレートのルート
    └── utils.rs  // Rustを使って作業することを簡単にする
```

`pkg`ディレクトリ
```shell
pkg/
├── package.json
├── README.md
├── wasm_game_of_life_bg.wasm
├── wasm_game_of_life.d.ts
└── wasm_game_of_life.js
```
`.wasm`ファイルはRustのソースからRustコンパイラによって生成されたWebAssemblyのバイナリ。
Rustの関数とデータ全てのWasmにコンパイルされたバージョンを含む。

`wasm-game-of-life/www`ディレクトリ
```shell
wasm-game-of-life/www/
├── bootstrap.js        
├── index.html          // ルートHTMLファイル
├── index.js            // JavaScriptファイルのエントリーポイント
├── LICENSE-APACHE
├── LICENSE-MIT
├── package.json        // wasm-pack-template パッケージのバージョンである hello-wasm-pack の依存関係が事前に設定 
├── README.md
└── webpack.config.js   // webpackを環境設定
```
## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

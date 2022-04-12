# Web Assembly とは？
[参考文献](https://moshg.github.io/rustwasm-book-ja/what-is-webassembly.html#webassembly%E3%81%A8%E3%81%AF)

WebAssembly (wasm) とは単純な機械モデルと広範な仕様を伴った実行可能形式のことです。wasmはポータブルかつコンパクトで、ネイティブと同じかそれに近いスピードで実行されるように設計されています。

## WebAssemblyはウェブだけのためのもの？
今のところ一般にJavaScriptとウェブのコミュニティに注目を集めていますが、wasmはホスト環境について一切の仮定をしません。それゆえ、wasmは将来様々な文脈で使用される「ポータブルな実行ファイル」形式になるというと推測するのは理にかなっています。しかし今現在、wasmはたいてい (ウェブ上やNode.js上のものを含めて) 多くの種類を持つJavaScript (JS) と関連付けられています。

## 何を学ぶのか？
- WebAssemblyへコンパイルするためにRustのツールチェインをセットアップする方法。
- Rust、WebAssembly、JavaScript、HTML、CSSから成る複数言語プログラムを開発するワークフロー。
- RustとWebAssembly両方の強みを、そしてさらにJavaScriptの強みを最大限に生かすAPIをデザインする方法。
- RustからコンパイルされたWebAssemblyモジュールをデバッグする方法。
- Rust and WebAssemblyのプログラムをより速くするためにプロファイルする方法。
- Rust and WebAssemblyのプログラムをより小さくし、ネットワーク越しのダウンロードをより速くするためのサイズプロファイルの方法。
# Webページを公開する

**cargコマンドの素晴らしさ**　　
cargoコマンドが、パッケージの正しいバージョンを
- ダウンロード
- ビルド
- 要求があれば、更新する

## crateとは
Rustのパッケージは、
- ライブラリ
- 実行ファイル
のことを**crate** という。　　

webフレームワークは
- actix-web

シリアライズを行う
- serde

use宣言を行う
javascriptでいう`import文`みたいなもの？
```rust
use actix_web::{web, App, HttpResponse, HttpServer};
```



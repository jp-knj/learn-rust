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

u64型の２つのフィールドnとmを持つ、新しい型GcdParametersを宣言。
フィールドの型は、gcd関数の引数の型と一致
```rust
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}
```

## #[something]とは

ハンドラ関数にweb::Form<GcdParameter>の値が使える。

`format!`と`println!`マクロの違いとは？

標準出力に出すのではなく、文字列として返す。
```rust
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0|| form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the gcd with zero is boring");
    }
    let response = format!("the greatest common divisor of the numbers {} and {} \
    is <b>{}</b> \n", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}
```
# 参照
所有権を持たないポインタ型、参照

Rustはある値に対する参照を作ることを借用と呼ぶ。
- ある値に対する参照を作ることができる、借用と呼ぶ。

## 値への参照
- 共有参照
  - 参照先を読むことができるが変更することはできない。
- 可変参照
  - 値を読み出すことも変することもできる。
- 値渡し
  - 値の所有権を移動するような方法で冠すへ値を渡すこと
- 参照渡し
  - 関数に値の参照を渡すこと
---
theme: default
title: ASTのフラット化によるパフォーマンスの差異と考察
fonts:
  sans: M PLUS 1
  serif: M PLUS 1
  mono: Shippori Mincho
class: text-center
drawings:
  persist: false
transition: slide-left
mdc: true
---

# テーマ研究: 研究発表

## ASTのフラット化による探索パフォーマンスの差異とその考察

<p class="!opacity-100 !mt-10">学籍番号: 2101105460 <span class="text-xl">市村 悠馬</span></p>

---

# 目次

1. テーマの説明
2. 計測項目と手法
3. 得られた結果と考察
4. まとめ

---
layout: header
---

# テーマの説明

::body::

<p class="text-[1.3em]">AST（抽象構文木）のフラット化による探索パフォーマンスの差異を計測し考察する</p>

---
layout: header
---

# 背景説明

::body::

<span class="text-[1.8rem] font-bold">Rust製JavaScriptツールチェインであるdeno_lintで実際に行われているパフォーマンスチューニングの手法</span>

---

# 背景説明

## AST(抽象構文木)とは

- プログラムコードを木構造で表したもの
- 静的な解析はこの木を読むことで行う

`if(condition) { foo(); }`

```js
const ast = {
  type: "IfStatement",
  test: { type: "Identifier", value: "condition", optional: false },
  consequent: {
    type: "BlockStatement",
    stmts: [
      { type: "ExpressionStatement", expression: { /*...*/ } }
    ]
  }
};
```

---

# 背景説明

- deno_lintではJSプラグインでの拡張を提供しているため、RustとJSの間でASTをやり取りしなければならない
- デシリアライズのオーバーヘッドが大きくて時間がかかる
- → 「フラット化」によってそれを解消する

---

# フラット化とは

AST構造をフラット（平坦）な配列に変換する手法

- 木構造をフラットな配列形式に変換
- 親子関係はインデックス番号で表現
  - 子のインデックス番号、兄弟のインデックス番号、親のインデックス番号を持つ
  - → n個目のノードのインデックス番号は`n * 4`で求まる
- 走査効率の向上が期待できる

---
layout: two-cols-header
---

# フラット化とは

::left::

## Pure-AST

```js
const ast = {
  type: "IfStatement",
  test: { type: "Identifier", value: "condition", optional: false },
  consequent: {
    type: "BlockStatement",
    stmts: [
      { type: "ExpressionStatement", expression: { /*...*/ } }
    ]
  }
};
```

::right::

## Flatten-AST

```js
// フラット化されたAST（配列とインデックスで関係性を表現）
const ast = {
  stringTable: ["", "IfStatement", "Identifier", /*...*/],
  properties: [{}, { test: 2, consequent: 3, /*...*/ }, /*...*/],
  nodes: [0, 0, 0, 0, 1, 2, 0, 0, /*...*/]
};
```

---
layout: header
---

# 研究内容の詳細

::body::

- フラット化前のAST構造とフラット化後のAST構造について、<span class="text-xl font-bold">同一の探索アルゴリズムを用いて性能比較</span>を行う
  - 同一のソースコードから生成したAST（通常・フラット化）に対して、同じ探索経路をたどる

<div class="mt-10">
  余裕があったら...<br/>
  なるべく大きなコード(例えばTSのchecker.tsなど)のASTで実験を行いたい
</div>

---
layout: two-cols-header
---

# 計測項目: 走査処理の総実行時間

::left::

<div class="ml-0">

## 計測対象
- 探索関数の開始から終了までの時間

## 理由
- パフォーマンス評価の基本指標
- 実用的な性能差の把握

## 手法
- Rustの`std::time::Instant`を使用
- 複数回の実行による平均値計測

</div>

::right::

```rust
use std::time::Instant;

fn measure_traverse_time(ast: &Ast) -> Duration {
    let start = Instant::now();
    traverse_ast(ast);
    let elapsed = start.elapsed();

    elapsed
}
```

---
layout: two-cols-header
---

# 計測項目: アロケーション回数

::left::

<div class="ml-0">

## 計測対象
- 走査中のメモリ割り当て回数

## 理由
- メモリ管理オーバーヘッドの評価
- 動的確保による性能影響の測定

## 手法
- カスタムアロケータによる計測
- `#[global_allocator]`属性を利用

</div>

::right::

```rust
struct CountingAllocator;

static ALLOCATION_COUNTER: AtomicUsize =
    AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATION_COUNTER.fetch_add(1,
            Ordering::SeqCst);
        System.alloc(layout)
    }
    // ...
}

#[global_allocator]
static ALLOCATOR: CountingAllocator =
    CountingAllocator;
```

---

# 得られた結果 -走査処理の総実行時間

## 実行時間の比較（5回測定）

| 実行回数 | 通常AST | フラットAST |
|--------|---------|-----------|
| <span class="text-xs">1回目</span> | <span class="text-xs">45.9µs</span> | <span class="text-xs"> 8.9µs </span> |
| <span class="text-xs">2回目</span> | <span class="text-xs">27.0µs</span> | <span class="text-xs">10.0µs</span> |
| <span class="text-xs">3回目</span> | <span class="text-xs">25.0µs</span> | <span class="text-xs"> 8.8µs</span> |
| <span class="text-xs">4回目</span> | <span class="text-xs">25.3µs</span> | <span class="text-xs"> 8.7µs</span> |
| <span class="text-xs">5回目</span> | <span class="text-xs">26.0µs</span> | <span class="text-xs">13.9µs</span> |
| **平均** | **29.84µs** | **10.06µs** |

<p class="text-xs !m-0">※ Release Profile（cargo run --release）での実行結果</p>
<p class="text-xs !m-0">※ コミットハッシュ<code>a38f4eeb</code>のソースコードでの結果</p>

---

# 得られた結果 -走査処理の総実行時間

## 実行時間の比較（5回測定）

| 実行回数 | 通常AST | フラットAST |
|--------|---------|-----------|
| **平均** | **29.84µs** | **10.06µs** |

<div class="mt-10">

<p class="text-6xl text-center">約66%向上</p>

</div>
---

# 考察: パフォーマンス向上の理由

<p class="text-[2em] !mt-20 !mb-10 !opacity-100">メモリアクセスの局所性が高まったから</p>

- pure-AST: 各ノードをヒープメモリ上に個別にアロケート
  - キャッシュミスが頻発
- flatten-AST: データをメモリ上で連続的に配置
  - CPUキャッシュを有効活用

---

# まとめ

- フラット化前のAST構造とフラット化後のAST構造で同一の走査を行って性能を比較
- 計測項目は2種類
  - 走査処理の総実行時間
  - アロケーション回数
- 走査処理の総実行時間は約66%向上した
  - 理由はメモリアクセスの局所性が高まったからだと考えられる

---

## 参考文献

- Marvin Hagemeister. "Speeding up the JavaScript ecosystem - Rust and JavaScript Plugins". marvinh.dev. 2025. https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-11/. (参照 2025-06-24)
- Shiisaa Moriai. "Rustのメモリアロケーションをちょっとだけ掘ってみた". Qiita. 2020. https://qiita.com/moriai/items/4e2ec2d9c3b352394ef3. (参照 2025-07-03)

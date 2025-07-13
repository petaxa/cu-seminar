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

# ASTのフラット化によるパフォーマンスの差異と考察

テーマ研究発表

<div class="abs-br m-6 flex gap-2">
  <span>2025年7月21日</span>
</div>

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

<p class="text-[1.3em]">AST（抽象構文木）のフラット化によるパフォーマンスの差異を計測し考察する</p>

---

# 背景説明

## linterとは

- コードの品質チェックや問題検出を行うツール
- 構文エラー、潜在的バグ、スタイル違反などを発見
- 例: ESLint, TSLint

## Rust製ツールチェイン

- 高速で安全なlinterの実装が可能
- 例: deno_lint (DenoのJavaScript/TypeScriptリンター)

---

# Rust製ツールチェインでJSプラグインを提供する課題

- Rust側とJavaScript側でデータをやり取りする必要がある
- 特にAST（抽象構文木）の受け渡しが課題
- デシリアライズのオーバーヘッドが大きい

```js
// 一般的なAST構造（入れ子になっている）
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

# フラット化とは

AST構造をフラット（平坦）な配列に変換する手法

- 木構造をフラットな配列形式に変換
- 親子関係はインデックス番号で表現
  - 子のインデックス番号、兄弟のインデックス番号、親のインデックス番号を持つ
  - → n個目のノードのインデックス番号は`n * 4`で求まる
- 走査効率の向上が期待できる

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
- 計測項目は3種類
  - 走査処理の総実行時間
  - メモリ使用量
  - アロケーション回数
- 走査処理の総実行時間は約66%向上した
- 理由はメモリアクセスの局所性が高まったからだと考えられる

---

## 参考文献

- Marvin Hagemeister. "Speeding up the JavaScript ecosystem - Rust and JavaScript Plugins". marvinh.dev. 2025. https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-11/. (参照 2025-06-24)
- Shiisaa Moriai. "Rustのメモリアロケーションをちょっとだけ掘ってみた". Qiita. 2020. https://qiita.com/moriai/items/4e2ec2d9c3b352394ef3. (参照 2025-07-03)

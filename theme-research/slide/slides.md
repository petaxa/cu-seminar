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
layout: two-cols
---

# テーマの説明

::right::

<div class="ml-4">
  <h3>AST（抽象構文木）のフラット化によるパフォーマンスの差異を計測し考察する</h3>
  <img src="https://mermaid.ink/img/pako:eNptkU9PwzAMxb9KlBOI9QPkBBpDQmxo0_6oEodcG9poS5wpcYfG0HdHaVkn4JT4vZ_fk5MLqs4J5ugbRr8DXO_rzp9pxJItedVURmvrkGFA7yQl9B3YQC6eZSUiGLDKaXCbiEck2ErDCiFdYOzFYG2UsB3ShN8wcNHSysIwgQ8WE3yQ9ZqeX3EEMUahZ47thml-NFJUiysRZYPmHa1HFDgXWD0w-Yv9e7q3wuvs_d_6K8aJXpCDZIbWF3rtPFscJOtxCW4fTpd69g1609dZzmnTFPB9yTRF0efeHNAaXMjZMsxQthoyfGLYm0ZlOJdYixPQwZJzqFTuO9sLXKHvJEsvn97R_jCkP2lzgP8" alt="ASTとフラット化のイメージ図" />
</div>

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

# 研究内容の詳細

フラット化前のAST構造とフラット化後のAST構造について、同一の探索アルゴリズムを用いて性能比較を行う
同一のソースコードから生成したAST（通常・フラット化）に対して、同じ探索経路をたどる

余裕があったら...
なるべく大きなコード(例えばTSのchecker.tsなど)のASTで実験を行いたい

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

# 計測項目: メモリ使用量

::left::

<div class="ml-0">

## 計測対象
- 各AST構造のメモリ消費量

## 理由
- データ構造の効率性評価
- メモリ使用効率の比較

## 手法
- `std::mem::size_of_val`を再帰的に使用
- 実際のメモリ配置を考慮した計測

</div>

::right::

```rust
use std::mem::size_of_val;

fn analyze_ast_memory(ast: &Ast) -> usize {
    size_of_val(ast)
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

※ Release Profile（cargo run --release）での実行結果
※ 現在のソースコードでの結果

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

- 「キューにpush」が「ポインタへのアクセス」に置き換わったから

---

# まとめ

- **検証結果**: ASTのフラット化は走査性能を大幅に向上させる
- **効果**: メモリ使用効率とデータアクセス効率の両面で改善
- **応用**: JavaScript/TypeScriptリンターなどのツールチェーン高速化に有効
- **今後の展望**: より多様なケースでの検証と最適化手法の研究

## 参考文献

- Marvin Hagemeister. "Speeding up the JavaScript ecosystem - Rust and JavaScript Plugins". marvinh.dev. 2025
- Shiisaa Moriai. "Rustのメモリアロケーションをちょっとだけ掘ってみた". Qiita. 2020

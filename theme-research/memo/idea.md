# アイデアを書き連ねる

[Plugin System in Rust based JavaScript / TypeScript Linters](https://speakerdeck.com/unvalley/typescript-linters) で触れられていた、AST をフラット化することで JSON を Deserialization する際のオーバーヘッドを排除する話がめちゃくちゃおもろそうだったので、これをテーマに性能検証がしたい

[動画: 【TSKaigi2025】トグルルーム Day1](https://www.youtube.com/live/aaG8Wczv_V4?si=u-VNbM8ZwkXv4ewn&t=23072)

コード例は 自作の AST で補いながら示す。(TODO: できていないので、やる)

## Plugin System in Rust based JavaScript / TypeScript Linters の内容をかみ砕く

- Rust 製 Linter で JS / TS Plugins を提供するときにどうするのかという話をしている
- Rust 製 Linter で JS / TS Plugins を提供するとき、Linter 内で JS Engine を持つ必要がある
  - これを行っているのが [deno_lint の custom rules](https://docs.deno.com/runtime/reference/lint_plugins/)
- つまり、Rust ⇔ JS Engine 間のやり取りが発生する
- そのとき、Rust で生成した AST を JSON 化 → JS Engine で `JSON.parse()` の処理がとても重いのが課題
  1. [Rust] User の JS コードを静的に読み取り、パースして AST を出力
  2. [Rust] Rust で書かれた AST を [serde_json](https://docs.rs/serde_json/latest/serde_json/) で JSON に変換
  3. [JS Engine] 変換された JSON を受け取り、`JSON.parse()` で JS Object に変換
- AST のトラバーサルは Rust では 単純なアルゴリズムを並列化することで高速に処理しているが、JSEngine ではそれができないため遅い世界に逆戻りしてしまう
- → AST のフラット化によって小さいデータにして渡したい
  - 最終的には数値のみが入った配列にして渡す

### AST とフラット化の具体的なコード

`if(condition) { foo() }` という JS コードの AST をフラット化する

#### AST

```js
const ast = {
  type: "IfStatement",
  test: { type: "Identifier", name: "condition" },
  consequent: {
    type: "BlockStatement",
    body: [
      [
        {
          type: "ExpressionStatement",
          expression: { type: "CallExpression" },
        },
      ],
    ],
  },
  alternate: null,
};
```

#### ID による参照

各ノードを入れ子にするのをやめ、配列の位置情報で親子関係を表す
例えば、`test: 2,` は test プロパティに ast[2] つまり `{ type: "Identifier", name: "condition" }` が当てはまるということ。

```js
const ast = [
  { type: "" },
  {
    type: "IfStatement",
    test: 2,
    consequent: 3,
    alternate: 0,
  },
  { type: "Identifier", name: "condition" },
  { type: "BlockStatement", body: [4] },
  { type: "ExpressionStatement", expression: 5 },
  { type: "CallExpression" },
];
```

#### プロパティの分離

配列の各要素オブジェクトの構造をそろえるため、type、child、next、parent のみをプロパティに持つようにし、properties 配列に置いた実態の添え字を格納する

```js
const ast = {
  properties: [
    {},
    { test: 2, consequent: 3, alternate: 0 },
    { name: "condition" },
    { body: [4] },
    { expression: 5 },
  ],
  nodes: [
    { type: "", child: 0, next: 0, parent: 0 },
    { type: "IfStaetment", child: 2, next: 0, parent: 0 },
    { type: "Identifier", child: 0, next: 3, parent: 1 },
    { type: "BlockStatement", child: 4, next: 0, parent: 1 },
    { type: "ExpressionStatement", child: 5, next: 0, parent: 3 },
    { type: "CallExpression", child: 0, next: 0, parent: 4 },
  ],
};
```

#### 文字列テーブルによる最適化 -文字列テーブルプロパティの追加

より効率的な構造(e.g. Int8Array) を目指すため、type の実態を stringTable 配列に置き、添え字を格納する

```js
const ast = {
  stringTable: ["", "IfStatement", "Identifier", "BlockStatement", "ExpressionStatement", "CallExpression"],
  properties: [
    {},
    { test: 2, consequent: 3, alternate: 0 },
    { name: "condition" },
    { body: [4] },
    { expression: 5 },
  ],
  nodes: [
    { type: 0, child: 0, next: 0, parent: 0 },
    { type: 1, child: 2, next: 0, parent: 0 },
    { type: 2, child: 0, next: 3, parent: 1 },
    { type: 3, child: 4, next: 0, parent: 1 },
    { type: 4, child: 5, next: 0, parent: 3 },
    { type: 5, child: 0, next: 0, parent: 4 },
  ],
};
```

#### 文字列テーブルによる最適化 -nodes のプロパティを配列の位置情報で表現

nodes 配列を完全にフラット化し、index * 4 でノードの開始位置を表す
index とは前項での node 配列の index 番号(添え字)

```js
const ast = {
  stringTable: ["", "IfStatement", "Identifier", "BlockStatement", "ExpressionStatement", "CallExpression"],
  properties: [
    {},
    { test: 2, consequent: 3, alternate: 0 },
    { name: "condition" },
    { body: [4] },
    { expression: 5 },
  ],
  nodes: [
    0, 0, 0, 0,
    1, 2, 0, 0,
    2, 0, 3, 1,
    3, 4, 0, 1,
    4, 5, 0, 3,
    5, 0, 0, 4,
  ],
};
```

## 基本方針を形作りたい

AST をフラット化(IDによる参照でAST配列化、プロパティの分離でノード形状を統一、文字列テーブルによる最適化)したとき、どのくらいの性能差がでるのかを定量的に計測、理論値の算出、計測結果から得られる考察を行う
AST のフラット化によってできた数値のみの配列と単なる AST を単純な探索アルゴリズムで探索し、その差を確認する

## 参考になりそうな文献

<https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-11/>
[deno_lint Plugins のインフラ実装(正確にはどこかわからんが、deno_core にあるらしい)](https://github.com/denoland/deno_core/blob/main/core/00_infra.js)

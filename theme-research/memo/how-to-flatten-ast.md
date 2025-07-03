# 結局、フラット化の手法ってどういう手順になっているの？

Speeding up the JavaScript ecosystem - Rust and JavaScript Plugins では中略が入っているため、手順に従って全文をやってみる。

[deno_lint では Span と ctxt を削除していた](https://github.com/denoland/deno_lint/blob/4a56a4939012b533ecd775f3bc2c36812af235c6/src/swc_util.rs#L319)ため、これらの探索は行わない。
optional は 削除していなさそうなので残す。
参考元では Identifier の name となっているプロパティについて、swc では value になっているのでそのままのプロパティ名を使う。
どうように、BlockStatement の body も swc のものを採用し stmt とする。

## 元の AST

参考元と同様に、`if(condition) { foo(); }` に対する AST を利用する

```js
const ast = {
  type: "IfStatement",
  test: {
    type: "Identifier",
    value: "condition",
    optional: false,
  },
  consequent: {
    type: "BlockStatement",
    stmts: [
      {
        type: "ExpressionStatement",
        expression: {
          type: "CallExpression",
          callee: {
            type: "Identifier",
            value: "foo",
            optional: false,
          },
          arguments: [],
          type_arguments: null,
        },
      },
    ],
  },
  alternate: null,
};
```

## ID による参照

index 0 は空の値のプレースホルダとする。固定で `{ type : "" }`

```js
const ast = [
  { type: "" },
  {
    type: "IfStatement",
    test: 2,
    consequent: 3,
    alternate: 0,
  },
  {
    type: "Identifier",
    value: "condition",
    optional: false,
  },
  {
    type: "BlockStatement",
    stmts: [4],
  },
  {
    type: "ExpressionStatement",
    expression: 5,
  },
  {
    type: "CallExpression",
    callee: 6,
    arguments: [],
    type_arguments: 0,
  },
  {
    type: "Identifier",
    value: "foo",
    optional: false,
  },
];
```

## プロパティの分離

nodes の要素が持つ各プロパティは、ほかの nodes 要素との相対的な位置関係を表す。
つまり、どのような順でノードをたどればよいのかという情報のみを保持し、そのノードのプロパティは同一の index 番号で properties に保存しておく。

整理のためにこれらから導かれる事実を陳列すると、

- properties の要素数と nodes の要素数は必ず同じになる
- nodes[i] のプロパティは properties[i] に格納されている
- nodes 要素の child, next に格納されている index 番号をたどればすべての nodes 要素を正しい順で探索できる

```js
const ast = {
  properties: [
    {},
    {
      test: 2,
      consequent: 3,
      alternate: 0,
    },
    {
      value: "condition",
      optional: false,
    },
    {
      stmts: [4],
    },
    {
      expression: 5,
    },
    {
      callee: 6,
      arguments: [],
      type_arguments: 0,
    },
    {
      value: "foo",
      optional: false,
    },
  ],
  nodes: [
    { type: "", child: 0, next: 0, parent: 0 },
    { type: "IfStatement", child: 2, next: 0, parent: 0 },
    { type: "Identifier", child: 0, next: 3, parent: 1 },
    { type: "BlockStatement", child: 4, next: 0, parent: 1 },
    { type: "ExpressionStatement", child: 5, next: 0, parent: 3 },
    { type: "CallExpression", child: 6, next: 0, parent: 4 },
    { type: "Identifier", child: 0, next: 0, parent: 0 },
  ],
};
```

## 文字列テーブルによる最適化 -文字列テーブルプロパティの追加

可変長の値を削除する(バッファで直接トラバーサルするには必要である)ため、type には stringTable への index 番号を格納する。
stringTable は重複の無い type 文字列の配列である(おそらく)
フラット化する際に stringTable になかったら登録してその index を、あったらその index を貼るという処理を行えばよいので、要素数を node 等と一致させる必要はないはずである。

```js
const ast = {
  stringTable: [
    "",
    "IfStatement",
    "Identifier",
    "BlockStatement",
    "ExpressionStatement",
    "CallExpression",
  ],
  properties: [
    {},
    {
      test: 2,
      consequent: 3,
      alternate: 0,
    },
    {
      value: "condition",
      optional: false,
    },
    {
      stmts: [4],
    },
    {
      expression: 5,
    },
    {
      callee: 6,
      arguments: [],
      type_arguments: 0,
    },
    {
      value: "foo",
      optional: false,
    },
  ],
  nodes: [
    { type: 0, child: 0, next: 0, parent: 0 },
    { type: 1, child: 2, next: 0, parent: 0 },
    { type: 2, child: 0, next: 3, parent: 1 },
    { type: 3, child: 4, next: 0, parent: 1 },
    { type: 4, child: 5, next: 0, parent: 3 },
    { type: 5, child: 6, next: 0, parent: 4 },
    { type: 2, child: 0, next: 0, parent: 0 },
  ],
};
```

## 文字列テーブルによる最適化 -nodes のプロパティを配列の位置情報で表現

node 要素のプロパティ名を外し、`4 * index` で要素を算出できるようにする。
フラット化することでより効率的なデータ構造になり、`4 * index` で計算できるため探索ノードのオフセットを記録しておく必要もないのでただリソースを削減できる。

```js
const ast = {
  stringTable: [
    "",
    "IfStatement",
    "Identifier",
    "BlockStatement",
    "ExpressionStatement",
    "CallExpression",
  ],
  properties: [
    {},
    {
      test: 2,
      consequent: 3,
      alternate: 0,
    },
    {
      value: "condition",
      optional: false,
    },
    {
      stmts: [4],
    },
    {
      expression: 5,
    },
    {
      callee: 6,
      arguments: [],
      type_arguments: 0,
    },
    {
      value: "foo",
      optional: false,
    },
  ],
  nodes: [
    0, 0, 0, 0,
    1, 2, 0, 0,
    2, 0, 3, 1,
    3, 4, 0, 1,
    4, 5, 0, 3,
    5, 6, 0, 4,
    2, 0, 0, 0,
  ],
};
```

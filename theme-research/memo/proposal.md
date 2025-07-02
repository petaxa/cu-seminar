# AST のフラット化による移動回数、探索時間の変化を計測し、アルゴリズムの構造的特徴の面から理由の考察を行う

## 背景

Rust 製 JS/TS Linter が JS/TS Plugins を提供する際、Rust から JS Engine へユーザーのソースコードの AST を受け渡す必要がある。
同時に JS Engine 上で AST を表した JSON をデシリアライズすることが必要になる。
これは実行速度の面で大きなオーバーヘッドであり、これらを提供している deno_lint Plugins[^1] はソースコードの木構造をフラット化することで探索スピードを向上させている。[^2]

## 今回取り組むこと

今回のテーマ研究では AST とそれをフラット化したデータに対して同一の探索アルゴリズムを実行し、その移動回数や探索時間を計測し、差異の評価と理由の考察を行う。
実装は AST 用、フラットAST用の二つを用意し、それらはまったく同じような探索経路を経る必要がある(同一の探索アルゴリズムを利用する)。

## 簡単な調査結果

TSKaigi での unvalley 氏のセッション[^3][^4]を参考に、AST をフラット化したときのオブジェクト構造を示す。
今回はコード 1 の AST をフラット化する。
コード 2 は コード 1 の AST であり、コード 3 はコード 2 の AST を情報を保ったままフラット化したものである。
各ノードの入れ子構造を解消することで配列で表現し、配列の位置情報で親子関係を表している。また、配列の各要素の構造を統一するため、適宜複数の配列に分割し、規則に従ってインデックス番号を持つことで木構造を表現している。
性能測定の際は checker.ts[^5] など、極端に大きな JavaScript コードの AST に対して実行することにより、明確な差が出ると考えている。

```js
// コード 1: パース対象の JS コード
if(condition) { foo(); }
```

```js
// コード 2: フラット化前の AST
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

```js
// コード 3: フラット化後の AST
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

## 引用 / 参考文献

[^1]: https://docs.deno.com/runtime/reference/lint_plugins/
[^2]: Marvin Hagemeister. "Speeding up the JavaScript ecosystem - Rust and JavaScript Plugins". marvinh.dev. 2025. https://marvinh.dev/blog/speeding-up-javascript-ecosystem-part-11/. (参照 2025-06-24)
[^3]: unvalley. "Plugin System in Rust based JavaScript / TypeScript Linters". speakerdeck. 2025. https://speakerdeck.com/unvalley/typescript-linters. (参照 2025-06-24)
[^4]: TSkaigi. "【TSKaigi2025】トグルルーム Day1". YouTube. 2025. https://www.youtube.com/live/aaG8Wczv_V4?si=u-VNbM8ZwkXv4ewn&t=23072. (参照 2025-06-24)
[^5]: microsoft. "TypeScript/src/compiler/checker.ts". GitHub. 2025. https://github.com/microsoft/TypeScript/blob/main/src/compiler/checker.ts. (参照 2025-06-24)

---
theme: default
title: CU seminar -linked list / cdl list
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

# 輪講資料 10 章 / 11 章

## 線形連結リスト / 循環重連結リスト

<br />
<p>学籍番号: 2101105460 <span class="text-xl">市村 悠馬</span></p>

---

# 目次

- 線形連結リスト
- 循環重連結リスト
  - 線形連結リストとは
  - 特徴
  - 用語の説明
  - 実装方法
- まとめ

<div class="mt-[2rem]">
  <p>スライド、サンプルコード</p>
  <img class="w-5rem" src="./repo-QR.png" alt="今回のスライド、サンプルコードが含まれるGitHubリポジトリへリンクするQRコード画像" />
</div>

---
layout: intro
---

# 線形連結リスト

- 線形連結リストとは
- 特徴
- 用語の説明
- 実装方法
  - セルの挿入
  - セルの削除
  - 実際に動かすと

---
layout: header-intro
---

# 線形連結リスト

## 線形連結リストとは

::content::

### 各要素をポインタで連結させたリスト

<div class="flex items-center">
  <Cell width="90px" />
  <carbon-arrow-right class="w-[2rem] text-3xl"/>
  <Cell width="90px" />
  <carbon-arrow-right class="w-[2rem] text-3xl"/>
  <Cell width="90px" />
  <carbon-arrow-right class="w-[2rem] text-3xl"/>
  <Cell width="90px" />
</div>

<p>「<span class="underline decoration-2 decoration-[#88b3f7]">値</span>」と「<span class="underline decoration-2 decoration-[#f79388]">次のセルへのポインタ</span>」を持つ<span class="font-bold">セル</span>をつなげていく</p>

---
layout: header-intro
---

# 線形連結リスト

## 線形連結リストとは

::content::

<div class="flex gap-[2rem] center">
  <p class="text-3xl font-bold">単方向連結リスト</p>
  <p class="text-3xl font-bold">重連結リスト</p>
  <p class="text-3xl font-bold">循環連結リスト</p>
</div>

---
layout: header-intro
---

# 線形連結リスト

## 特徴

::content::

- <span class="text-2xl">低い計算量で挿入、削除が実行できる</span>
- <span class="text-2xl">メモリを多く消費する</span>
- <span class="text-2xl">物理的なメモリの位置に制約がない</span>

<!--
配列だと O(n) かかる挿入や削除をO(1) (ポインタの張替えのみ)で実行できる

配列は物理的なメモリの位置を配列の中の位置情報として扱う
連結リストはそれを値として持つことで物理的な制約から開放される
 -->

---
layout: two-cols-header
---

# 線形連結リスト

## 用語の説明

::left::

### セル

<Cell width="80px" />

リストの要素となる構造体
<p>「<span class="underline decoration-2 decoration-[#88b3f7]">値</span>」と「<span class="underline decoration-2 decoration-[#f79388]">次のセルへのポインタ</span>」を持つ</p>
::right::

```c
typedef struct cell
{
  int value;
  struct cell *next;
} Cell;
```

---
layout: intro
---

# 実装方法

## 線形連結リスト(単方向連結リスト)

---
layout: two-cols-header
---

# 線形連結リスト(単方向連結リスト)

## セルの挿入

セル A の後に新規セルを挿入する場合、

::left::

0. **セル A のポインタ、新規セルの値を受け取る**
1. **新規セル用のメモリ確保**
2. **新規セルの「データ」を格納**
3. **新規セルの「次のセルへのポインタ」を格納**
   1. セル A の「次のセルへのポインタ」の値を取得
   2. 取得した値を新規セルの「次のセルへのポインタ」へ格納
4. **セル A の「次のセルへのポインタ」を更新**
   1. 新規セルのポインタを格納

::right::

```c
void insert_cell(Cell **ponter, int value)
{
  Cell *new_cell = malloc(sizeof *new_cell);
  if (!new_cell)
    return;

  // 挿入セルに値を格納
  new_cell->value = value;

  // 挿入セルの次のセルは挿入前に挿入位置にいたセルとする
  new_cell->next = *ponter;

  // 挿入位置に存在するセルを挿入セルに書き換え
  *ponter = new_cell;
}
```

---
layout: two-cols-header
---

# 線形連結リスト(単方向連結リスト)

## セルの削除

セル A → セル B となるリストからセル B を削除する場合、

::left::

0. **セル B のポインタを受け取る**
1. **セル A の「次のセルへのポインタ」を更新**
   1. セル B の「次のセルへのポインタ」の値を取得
   2. 取得した値をセル A の「次のセルへのポインタ」へ格納
2. **セル B のメモリ領域を解放**

::right::

```c
void delete_cell(Cell **pointer)
{
  Cell *target = *pointer;

  // 削除位置に存在するセルを削除セルの次のセルに書き換え
  *pointer = target->next;

  // 削除セルのメモリ空間を開放
  free(pointer);
}
```

---
layout: two-cols-header
---

# 線形連結リスト(単方向連結リスト)

## 実際に動かすと

::left::

```c
int main()
{
  Cell *head = NULL;

  // リストに値を追加
  puts("リストに値を追加");
  insert_cell(&head, 10);
  insert_cell(&head->next, 20);
  insert_cell(&head->next->next, 30);
  insert_cell(&head->next->next->next, 40);
  print_list(head);

  // リストから値を削除
  puts("リストから 3 番目の値を削除");
  delete_cell(&head->next->next);
  print_list(head);

  // メモリを解放
  free_list(head);

  return 0;
}
```

::right::

```txt
リストに値を追加
10 -> 20 -> 30 -> 40 -> NULL
リストから 3 番目の値を削除
10 -> 20 -> 40 -> NULL
```

サンプルコード

<img class="w-5rem" src="./linked-QR.png" alt="線形連結リストのサンプルコードへリンクするQRコード画像" />

---
layout: intro
---

# 循環重連結リスト

- 循環重連結リストとは
  - 循環リスト
  - 重連結リスト
- 特徴
- 用語の説明
- 実装方法
  - リストのセットアップ
  - ノードの挿入
  - ノードの削除
  - 実際に動かすと

---
layout: header-intro
---

# 循環重連結リスト

## 循環重連結リストとは

::content::

<p class="text-3xl"><span class="font-bold">循環リスト</span> と <span class="font-bold">重連結リスト</span> を組み合わせたもの</p>

---
layout: header-intro
---

# 循環重連結リスト

## 循環重連結リストとは

::content::

### **循環リスト**

<p class="text-2xl">単方向連結リストに、末尾ノードから先頭ノードへの連結が加わったリスト</p>

---
layout: header-intro
---

# 循環重連結リスト

## 循環重連結リストとは

::content::

### **重連結リスト**
<p class="text-2xl">先頭と末尾を除くすべてのノードがその前後のノードと連結しているリスト</p>

---
layout: header-intro
---

# 循環重連結リスト

## 特徴

::content::

- <span class="text-2xl">どの要素から探索をはじめても一周できる</span>
- <span class="text-2xl">１つ前の要素を値の参照だけで確認できる</span>

<!--
単方向連結リストの場合、ひとつ前を見るには一周移動しなければならなかった
 -->
---
layout: two-cols-header
---

# 循環重連結リスト

## 用語の説明

::left::

### ノード

単方向連結リストではセルと呼んでいたものと同義。<br />輪講資料の変数名に倣う。

<Cell width="80px" />

::right::

```c
typedef struct node
{
  int value;
  struct node *prev;
  struct node *next;
} Node;
```

---
layout: two-cols-header
---

# 循環重連結リスト

## 用語の説明

::left::

### prev

<p class="before:content-[''] before:ml-8">前のノードを格納するプロパティ</p>

### next

<p class="before:content-[''] before:ml-8">次のノードを格納するプロパティ</p>

::right::

```c
typedef struct node
{
  int value;
  struct node *prev;
  struct node *next;
} Node;
```

---
layout: intro
---

# 実装方法

## 循環重連結リスト

---
layout: two-cols-header
---

# 循環重連結リスト

## リストのセットアップ

::left::

1. **新規ノードを作成**
2. **作成したノードを自己参照**
   1. next に自身をセット
   2. prev に自身をセット

::right::

```c
Node *setup_cdl_list(void)
{
  // 新規ノード作成
  Node *head = create_node(NULL);
  if (!head)
    return NULL;

  // 自己参照
  head->next = head->prev = head;

  return head;
}
```
<!--
ひとつ、ここで議論したい。
輪講資料には p94 に「ノードの値は空である」として話を進めているが、これは実装上の都合であり、アルゴリズムの要件ではないのではないか。
→ 全探索の起点を示す必要があり、いずれかの方法で head を指す必要がある。
→ のでアルゴリズムの要件に思える
-->

---
layout: two-cols-header
---

# 循環重連結リスト

## ノードの挿入

基準ノードの直前に挿入

::left::

0. **基準ノードのポインタ、新規ノードの値を受け取り**
1. **新規ノードを作成**
2. **新規ノードの prev、next をセット**
   - prev: 基準ノードの prev
   - next: 基準ノード
3. **「基準ノードの前ノード」の next を更新**
   - 新規ノードを設定
4. **基準ノードの prev を更新**
   - 新規ノードを設定

::right::

```c
Node *insert_before_benchmark(Node *benchmark, int value)
{
  // 新規ノード作成
  Node *insert_node = create_node(value);
  if (!insert_node)
    return NULL;

  // 新規ノードのprev、next をセット
  insert_node->prev = benchmark->prev;
  insert_node->next = benchmark;

  // 基準ノードの前ノードの next を更新
  benchmark->prev->next = insert_node;

  // 基準ノードの prev を更新
  benchmark->prev = insert_node;

  return insert_node;
}
```

---
layout: two-cols-header
---

# 循環重連結リスト

## ノードの挿入

基準ノードの直後に挿入

::left::

0. **基準ノードのポインタ、新規ノードの値を受け取り**
1. **基準ノードの next のポインタを取得**
2. **そのポインタの直前にノードを挿入**
   - `insert_before_benchmark` を利用

::right::

```c
Node *insert_after_benchmark(Node *benchmark, int value)
{
  // 基準ノードの後ノードの前に挿入すれば基準ノードの１つ後ろに挿入される
  return insert_before_benchmark(benchmark->next, value);
}
```

---
layout: header-intro
---

# 循環重連結リスト

## ノードの挿入

::content::

- <p class="text-xl">head の<span class="font-bold">直前</span>に挿入すると、<span class="font-bold">末尾</span>に挿入できる</p>
- <p class="text-xl">head の<span class="font-bold">直後</span>に挿入すると、<span class="font-bold">先頭</span>に挿入できる</p>

---
layout: two-cols-header
---

# 循環重連結リスト

## ノードの削除

::left::

0. **削除ノードのポインタを受け取り**
1. **「削除ノードの前ノード」の next を更新**
   - 削除ノードの next を設定
2. **「削除ノードの次ノード」の prev を更新**
   - 削除ノードの prev を設定
3. **削除ノードのメモリ領域を解放**

::right::

```c
void remove_node(Node *node)
{
  if (!node)
    return;

  // 前ノードの next を更新
  node->prev->next = node->next;

  // 後ノードの prev を更新
  node->next->prev = node->prev;

  free(node);
}
```

---
layout: two-cols-header
---

# 循環重連結リスト

## 実際に動かすと

::left::

```c
int main(void)
{
  Node *head = setup_cdl_list();
  if (!head) return 1;

  // リスト末尾に追加
  puts("リスト末尾に追加");
  insert_before_benchmark(head, 1); // 1、2、3 を追加

  // リスト 2 番目のノードの後ろに追加
  puts("リストに追加");
  insert_after_benchmark(second, 99); // second: 2 番目のノードを表すポインタ

  // リスト 4 番目を削除
  puts("リスト 4 番目を削除");
  remove_node(fourth); // fourth: 4 番目のノードを表すポインタ

  // リストを開放
  free_dlist(head);
  return 0;
}
```

::right::

```txt
リスト末尾に追加
1 -> 2 -> 3 -> 0(head)
size = 3
リストに追加
1 -> 2 -> 99 -> 3 -> 0(head)
size = 4
リスト 4 番目を削除
1 -> 2 -> 99 -> 0(head)
size = 3
```

サンプルコード

<img class="w-5rem" src="./cdl-QR.png" alt="循環重連結リストのサンプルコードへリンクするQRコード画像" />

---
layout: header-intro
---

# まとめ

## 線形連結リスト

::content::

<ul>
  <li class="mb-[1rem]"><span class="text-2xl">要素の位置関係も値としてもつ</span></li>
  <li class="mb-[1rem]"><span class="text-2xl">バラバラにメモリ領域を確保</span></li>
  <li><span class="text-2xl">挿入や削除をポインタの張替えのみ(O(1))で実行</span></li>
</ul>

---
layout: header-intro
---

# まとめ

## 循環重連結リスト

::content::

<ul>
  <li class="mb-[1rem]">(線形連結リストのひとつである)<span class="text-2xl">循環リストと重連結リストを組み合わせたリスト</span></li>
  <li class="mb-[1rem]"><span class="text-2xl">１つ前の要素を値の参照だけで確認できる</span></li>
  <li><span class="text-2xl">どの要素から探索をはじめても一周できる</span></li>
</ul>

---
layout: header-intro
---

# 参考文献

::content::

松原雅文 山田敬三. C によるアルゴリズムとデータ構造. 森北出版. 2021.

tsudaryo1715. "第8回　リスト構造の応用と探索問題". Qiita. 2022. https://qiita.com/tsudaryo1715/items/12c4848028716ab015bb. (参照 2025-05-03)

田村仁. "連結リストを学ぶ -Python-". 田村研究室 プログラミング技法とデータ構造. 2011. https://leo.nit.ac.jp/~tamura/algorithm/lesson08.html. (参照 2025-05-03)

maec_lamar. "連結リストについてまとめてみた". Qiita. 2024. https://qiita.com/maec_lamar/items/709e52add85452565717. (参照 2025-05-10)

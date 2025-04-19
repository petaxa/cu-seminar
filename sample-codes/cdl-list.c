#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stddef.h>

typedef struct node
{
  int value;
  struct node *prev;
  struct node *next;
} Node;

/**
 * @brief セルを作成
 *
 * @param value 値
 *
 * @return Node* セルのポインタ
 */
static Node *create_node(int value)
{
  // メモリ確保
  Node *n = malloc(sizeof *n);
  if (!n)
    return NULL;

  // 値を格納
  n->value = value;
  return n;
}

/**
 * @brief cdl リストのセットアップ
 *
 * 空リストを作成し、prev、next に自分自身を指定
 *
 * @return Node* リスト先頭のポインタ
 */
Node *setup_cdl_list(void)
{
  // 新規ノード作成
  Node *head = create_node(0);
  if (!head)
    return NULL;

  // 自己参照
  head->next = head->prev = head;

  return head;
}

/**
 * @brief リストのメモリをすべて開放
 *
 * @param head リストの先頭のポインタ
 */
void free_dlist(Node *head)
{
  if (!head)
    return;

  // head の次(値のあるノード)から走査
  Node *current = head->next;
  while (current != head)
  {
    Node *tmp = current;
    current = current->next;
    free(tmp);
  }

  // 最後に head を開放
  free(head);
}

/**
 * @brief benchmark の直前にセルを挿入
 *
 * @param benchmark 基準セルのポインタ
 * @param value 挿入セルの値
 *
 * @return Node* 成功: 新ノードのポインタ 失敗: NULL
 */
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

/**
 * @brief benchmark の直前にセルを挿入
 *
 * @param benchmark 基準セルのポインタ
 * @param value 挿入セルの値
 *
 * @return Node* 成功: 新ノードのポインタ 失敗: NULL
 */
Node *insert_after_benchmark(Node *benchmark, int value)
{
  // 基準ノードの後ノードの前に挿入すれば基準ノードの１つ後ろに挿入される
  return insert_before_benchmark(benchmark->next, value);
}

/**
 * @brief 指定したノードを削除
 *
 * @param node 削除したいノード head を渡してはいけない
 *
 * @param node
 */
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

// head の削除対策に API を wrap
/**
 * @brief リストの先頭にノードを追加
 *
 * @param head リストの先頭のポインタ
 * @param value
 * @return Node*
 */
Node *push_head(Node *head, int value)
{
  return insert_after_benchmark(head, value);
}

/**
 * @brief リストの末尾にノードを追加
 *
 * @param head リストの先頭のポインタ
 * @param value
 * @return Node*
 */
Node *push_tail(Node *head, int value)
{
  return insert_before_benchmark(head, value);
}

/**
 * @brief リストの先頭ノードを削除
 *
 * @param head リストの先頭のポインタ
 */
void pop_head(Node *head)
{
  remove_node(head->next);
}

/**
 * @brief リストの末尾ノードを削除
 *
 * @param head リストの先頭のポインタ
 */
void pop_tail(Node *head)
{
  remove_node(head->prev);
}

/**
 * @brief リストの要素数を算出
 *
 * @param head リストの先頭のポインタ
 * @return size_t リストの要素数
 */
size_t list_size(const Node *head)
{
  size_t n = 0;
  for (const Node *cur = head->next; cur != head; cur = cur->next)
    ++n;
  return n;
}

/**
 * @brief head から走査して表示
 *
 * @param head リストの先頭のポインタ
 */
void print_list(const Node *head)
{
  for (Node *current = head->next; current != head; current = current->next)
    printf("%d ", current->value);
  puts("");
}

void print_size(const Node *head){
  printf("size = %zu\n", list_size(head));
}

int main(void)
{
  Node *head = setup_cdl_list();
  if (!head)
    return 1;

  // リスト末尾に追加
  puts("リスト末尾に追加");
  push_tail(head, 1);
  print_list(head);

  push_tail(head, 2);
  print_list(head);

  push_tail(head, 3);
  print_list(head);
  print_size(head);

  // リスト先頭に追加
  puts("リスト先頭に追加");
  push_head(head, 0);
  print_list(head);
  print_size(head);

  // リスト 2 番目に追加
  puts("リスト 2 番目に追加");
  Node *second = head->next->next;
  insert_before_benchmark(second, 99);
  print_list(head);
  print_size(head);

  // リスト末尾を削除
  puts("リスト末尾を削除");
  pop_tail(head);
  print_list(head);
  print_size(head);

  // リストを開放
  free_dlist(head);
  return 0;
}

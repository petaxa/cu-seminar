#include <stdio.h>
#include <stdlib.h>

typedef struct cell
{
  int value;
  struct cell *next;
} Cell;

/**
 * @brief リストに値を指定されたメモリ領域に挿入する。挿入前に指定されたメモリ領域に存在したセルは挿入セルの next となる。
 *
 * @param ponter 挿入セルの位置のメモリ領域を値に持つメモリのポインタ
 * @param value 挿入する値
 */
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

/**
 * @brief リストの値を削除する
 *
 * @param pointer 削除セルの位置のメモリ領域を値に持つメモリのポインタ
 */
void delete_cell(Cell **pointer)
{
  Cell *target = *pointer;

  // 削除位置に存在するセルを削除セルの次のセルに書き換え
  *pointer = target->next;
}

/**
 * @brief リストの内容をすべて表示
 *
 * @param head リストの先頭のポインタ
 */
void print_list(Cell *head)
{

  if (head == NULL)
  {
    printf("リストは空です。\n");
    return;
  }

  Cell *temp = head;
  while (temp != NULL)
  {
    printf("%d -> ", temp->value);
    temp = temp->next;
  }
  printf("NULL\n");
}

/**
 * @brief リストのメモリをすべて開放
 *
 * @param head リストの先頭のポインタ
 */
void free_list(Cell *head)
{
  Cell *temp;

  while (head != NULL)
  {
    temp = head;
    head = head->next;
    free(temp);
  }
}

int main()
{
  Cell *head = NULL;

  // リストに値を追加
  insert_cell(&head, 10);
  insert_cell(&head->next, 20);
  insert_cell(&head->next->next, 30);
  insert_cell(&head->next->next->next, 40);

  // リストの表示
  printf("リストの内容:\n");
  print_list(head);

  // リストから値を削除
  printf("\n30を削除します:\n");
  delete_cell(&head->next->next);
  print_list(head);

  // メモリを解放
  free_list(head);

  return 0;
}

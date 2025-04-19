#include <stdio.h>
#include <stdlib.h>

struct cell
{
  int value;
  struct cell *next;
};

/**
 *
 * @brief リストに値を指定されたメモリ領域に挿入する。挿入前に指定されたメモリ領域に存在したセルは挿入セルの next となる。
 *
 * @param ponter 挿入位置のメモリ位置
 * @param value 挿入する値
 */
void insert_cell(struct cell **ponter, int value)
{
  struct cell *new_cell = malloc(sizeof *new_cell);
  if (!new_cell)
    return;

  new_cell->value = value;
  new_cell->next = *ponter;
  *ponter = new_cell;
}

/**
 *
 * @brief リストの値を削除する
 *
 * @param pointer 削除するセルのポインタ
 * @param value
 */
void delete_cell(struct cell **pointer)
{
  struct cell *target = *pointer;
  *pointer = target->next;
}

/**
 *
 * @brief リストの内容をすべて表示
 *
 * @param head リストの先頭のポインタ
 */
void print_list(struct cell *head)
{

  if (head == NULL)
  {
    printf("リストは空です。\n");
    return;
  }

  struct cell *temp = head;
  while (temp != NULL)
  {
    printf("%d -> ", temp->value);
    temp = temp->next;
  }
  printf("NULL\n");
}

/**
 *
 * @brief リストのメモリをすべて開放
 *
 * @param head リストの先頭のポインタ
 */
void free_list(struct cell *head)
{
  struct cell *temp;

  while (head != NULL)
  {
    temp = head;
    head = head->next;
    free(temp);
  }
}

int main()
{
  struct cell *head = NULL;

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

// バブルソート

#include <stdio.h>
#include "shared/print.h"

#define ELE_NUM 10

/* 配列の初期状態を定義 */
static int ary[ELE_NUM] = {6, 9, 12, 7, 15, 23, 2, 10, 4, 20};

void main()
{
  int i, j, k;
  int temp;

  printf("ソート前\n");
  print_array(ary, ELE_NUM);

  for (i = 0; i < ELE_NUM; i++)
  {
    for (j = ELE_NUM - 1; j > i; j--)
    {
      if (ary[j - 1] > ary[j])
      {
        temp = ary[j - 1];
        ary[j - 1] = ary[j];
        ary[j] = temp;
      }
    }
  }

  printf("ソート後\n");
  print_array(ary, ELE_NUM);
}

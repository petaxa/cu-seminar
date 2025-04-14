#include "print.h"
#include <stdio.h>

void print_array(int array[], int length)
{
  for (int i = 0; i < length; i++)
  {
    printf("%d, ", array[i]);
  }
  printf("\n");
}

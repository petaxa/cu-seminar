#include <stdio.h>
#include <stdlib.h>

#define S_ONE 1
#define S_TWO 2
#define S_THREE 3
#define N_TEN 10
#define N_FIFTY 50

void insertionSort(int arr[], int n)
{
  int i, key, j;
  for (i = 1; i < n; i++)
  {
    key = arr[i];
    j = i - 1;

    while (j >= 0 && arr[j] > key)
    {
      arr[j + 1] = arr[j];
      j = j - 1;
    }
    arr[j + 1] = key;
  }
}

// Function to print an array
void printArray(int arr[], int size)
{
  int i;
  for (i = 0; i < size; i++)
    printf("%d ", arr[i]);
  printf("\n");
}

void gen_rand_array(int *dst, size_t n, int min, int max, unsigned seed)
{
  if (dst == NULL || n == 0)
    return;

  /* 範囲を整える */
  if (min > max)
  {
    printf("WARN: min > max. Swap min and max.");
    int tmp = min;
    min = max;
    max = tmp;
  }
  if (min == max)
  {
    printf("WARN: min = max. Added 1 to max.");
    max = min + 1;
  }

  unsigned range = (unsigned)(max - min + 1);
  if (range == 0)
    range = 1;

  if (seed == 0)
    seed = (unsigned)1;
  srand(seed);

  for (size_t i = 0; i < n; ++i)
    dst[i] = (int)(rand() % range) + min;
}

void run_sort(int n, int s)
{
  printf("N = %d, S = %d\n", n, s);

  int *arr = malloc((size_t)n * sizeof *arr);
  if (!arr)
  {
    perror("malloc failed");
    return;
  }

  gen_rand_array(arr, n, 1, 10 * n, s);

  printf("Original array: \n");
  printArray(arr, n);

  insertionSort(arr, n);

  printf("Sorted array: \n");
  printArray(arr, n);

  free(arr);
}

int main()
{
  // N_TEN
  run_sort(N_TEN, S_ONE);
  run_sort(N_TEN, S_TWO);
  run_sort(N_TEN, S_THREE);

  // N_FIFTY
  run_sort(N_FIFTY, S_ONE);
  run_sort(N_FIFTY, S_TWO);
  run_sort(N_FIFTY, S_THREE);

  return 0;
}

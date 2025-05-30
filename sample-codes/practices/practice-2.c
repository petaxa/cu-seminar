#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>

typedef struct
{
  size_t comparisons;
  size_t moves;
} SortStats;

SortStats insertionSort(int arr[], int n)
{
  SortStats st = {0, 0};

  for (size_t i = 1; i < n; ++i)
  {
    int key = arr[i];
    size_t j = i;

    st.moves++;

    while (j > 0)
    {
      st.comparisons++;
      if (arr[j - 1] > key)
      {
        arr[j] = arr[j - 1];
        st.moves++;
        --j;
      }
      else
      {
        break;
      }
    }
    arr[j] = key;
    st.moves++;
  }
  return st;
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

SortStats run_sort(int n, int s)
{
  printf("N = %d, S = %d\n", n, s);

  int *arr = malloc((size_t)n * sizeof *arr);
  if (!arr)
  {
    perror("malloc failed");
    SortStats failed_st = {0, 0};
    return failed_st;
  }

  gen_rand_array(arr, n, 1, 10 * n, s);

  SortStats st = insertionSort(arr, n);
  printf("比較回数: %d / 移動回数: %d \n", st.comparisons, st.moves);

  free(arr);

  return st;
}

void print_matrix_csv(size_t rows, size_t cols, const size_t *m)
{
  for (size_t i = 0; i < rows; ++i)
  {
    for (size_t j = 0; j < cols; ++j)
    {
      if (j)
        putchar(',');
      printf("%zu", m[i * cols + j]);
    }
    putchar('\n');
  }
}

int main()
{
  int s[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  int n[] = {10, 50, 100, 500, 1000, 5000, 10000};

  enum
  {
    S_LEN = sizeof s / sizeof s[0],
    N_LEN = sizeof n / sizeof n[0]
  };

  size_t comp[N_LEN][S_LEN];
  size_t move[N_LEN][S_LEN];

  for (size_t i = 0; i < S_LEN; ++i)
  {
    for (size_t j = 0; j < N_LEN; ++j)
    {
      SortStats st = run_sort(n[j], s[i]);
      comp[j][i] = st.comparisons;
      move[j][i] = st.moves;
    }
  }

  printf("\n比較回数 -csv\n");
  print_matrix_csv(N_LEN, S_LEN, &comp[0][0]);
  printf("\n移動回数 -csv\n");
  print_matrix_csv(N_LEN, S_LEN, &move[0][0]);

  return 0;
}

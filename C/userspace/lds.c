#include <stdio.h>
#include <stdlib.h>

static int *get_arr(int **arr)
{
	*arr = calloc(10, sizeof(int));
	return *arr;
}

static void print_arr(int *arr, int size)
{
	int i;
	for (i = 0; i < size; i++)
		printf("arr[%d]: %d\n", i, arr[i]);
}

static void populate_arr(int *arr, int size)
{
	for (int i = 0; i < size; i++)
		arr[i] = random();
}

static void linear_search(int *arr, int size)
{
	int num = 0;
	printf("Enter number to search: ");
	scanf("\n%d", &num);
	for (int i = 0; i < size; i++)
		if (arr[i] == num)
		{
			printf("Found %d at index: %d\n", num, i);	
			return;
		}
	printf("Number %d not found\n", num);
}

int main(int argc, char **argv, char **envp)
{
	for (int i = 0; envp[i]; i++)
		printf("envp[%d]: %s\n", i, envp[i]);
	int *arr = get_arr(&arr);
	populate_arr(arr, 10);
	print_arr(arr, 10);
	linear_search(arr, 10);
	return 0;
}

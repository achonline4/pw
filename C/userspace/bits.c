#include <stdio.h>
#include <limits.h>

static void print_bit_pattern(const char *msg, const unsigned int num)
{
	printf("#### %s\n", msg);
	for (int i = 0; i < (sizeof(num) * 8); i++)
		printf("%d", ((1 << (31 - i)) & num) > 0);
	printf("\n");
}

static unsigned int clear_bit(unsigned int num)
{
	int bitpos = -1;
	print_bit_pattern("before clearing bit", num);
	printf("Enter bit position to be cleared: ");
	scanf("\n%d", &bitpos);
	num &= ~(1 << bitpos);
	print_bit_pattern("after clearing bit", num);
	return num;
}

static unsigned int toggle_bit(unsigned int num)
{
	int bitpos = -1;
	print_bit_pattern("before toggling bit", num);
	printf("Enter bit position to be toggled: ");
	scanf("\n%d", &bitpos);
	num ^= (1 << bitpos);
	print_bit_pattern("after toggling bit", num);
	return num;
}

static void count_set_bits(unsigned int num)
{
	unsigned int count = 0;
	while (num)
	{
		num = num & (num - 1);
		count++;
	}
	printf("Number of set bits: %d\n", count);
}

static void _exp2(unsigned int num)
{
	int tv = (num  > 0) && ((num & (num - 1)) == 0);
	printf("Number (%d) is power of 2: %s\n", num, tv ? "Yes" : "No");
}

static void xor_swap(int *n1, int *n2)
{
	*n1 = *n1 ^ *n2;
	*n2 = *n1 ^ *n2;
	*n1 = *n1 ^ *n2;
}

int main(void)
{
	unsigned int a = UINT_MAX;
	print_bit_pattern("Test", a);
	a = clear_bit(a);
	a = toggle_bit(a);
	count_set_bits(a);
	_exp2(127);
	int n1 = 10, n2 = 20;
	printf("Before swap - n1: %d, n2: %d\n", n1, n2);
	xor_swap(&n1, &n2);
	printf("After swap - n1: %d, n2: %d\n", n1, n2);
	return 0;
}

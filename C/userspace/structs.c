#include <stdio.h>

struct abc {
	char a;
	char b;
	int c;
};

int main(void)
{
	printf("size: %lu, int: %lu\n", sizeof(struct abc), sizeof(int));
	return 0;
}

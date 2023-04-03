#include <stdio.h>


int main(int argc, char *argv[])
{
	int c;
	while ((c = getchar()) != EOF)
	{
		printf("c != EOF: %d\n", c != EOF);
		putchar(c);
	}
	return 0;
}

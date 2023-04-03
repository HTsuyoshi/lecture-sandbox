#include <stdio.h>

int main()
{
	int c;
	double nc = 0;
	while ((c = getchar()) != EOF)
		if (c == ' ' || c == '\t' || c == '\n') ++nc;
	printf("%lf", nc);
	return 0;
}


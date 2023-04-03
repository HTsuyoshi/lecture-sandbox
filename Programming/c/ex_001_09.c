#include <stdio.h>

int main()
{
	int c;
	double nc = 0;
	char t = 0;
	while ((c = getchar()) != EOF)
		if (c == ' ')
		{
			if (t) continue;
			putchar(c);
			t = 1;
		}
		else
		{
			putchar(c);
			t = 0;
		}
	return 0;
}


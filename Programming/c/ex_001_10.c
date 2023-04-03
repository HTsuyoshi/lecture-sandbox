#include <stdio.h>

int main()
{
	int c;
	double nc = 0;
	char t = 0;
	while ((c = getchar()) != EOF)
	{
		if (c == ' ')
			putchar('\b');
		else if (c == '	')
			putchar('\t');
		else if (c == '\\')
			putchar('\\');
		else
			putchar(c);
	}
	return 0;
}


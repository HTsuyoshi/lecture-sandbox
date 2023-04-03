#include <stdio.h>

int max(int a, int b)
{
	if (a>b) return a;
	return b;
}

int array_max(int a[], int n)
{
	int i, out;
	i = 0, out = 0;
	while(i < n)
	{
		out = max(a[i], out);
		++i;
	}
	return out;
}

int main()
{
	int c, i, nwhite, nother;
	int ndigit[10];

	nwhite = nother = 0;
	for (i = 0; i < 10; ++i)
		ndigit[i] = 0; while ((c = getchar()) != EOF)
		if (c >= '0' && c <= '9') ++ndigit[c - '0']; else if (c == ' ' || c == '\n' || c == '\t')
			++nwhite;
		else
			++nother;

	printf("digits =");
	for (i = 0; i < 10; ++i)
		printf(" %d", ndigit[i]);
	printf(", white space = %d, other = %d\n",
			nwhite, nother);

	int upper;
	upper = max(nother, max(nwhite, array_max(ndigit, 10)));
	printf("%d\n", upper);

	while (upper > 0)
	{
		putchar('|');
		for (i = 0; i < 10; ++i)
		{
			if (upper == ndigit[i])
			{
				putchar('*');
				--ndigit[i];
			}
			else
				putchar(' ');
		}
		if (upper == nwhite)
		{
			putchar('*');
			--nwhite;
		}
		else
			putchar(' ');
		if (upper == nother)
		{
			putchar('*');
			--nother;
		}
		else
			putchar(' ');
		putchar('\n');
		--upper;
	}

	putchar(' ');
	for (i = 0; i < 12; ++i)
		putchar('_');
	putchar('\n');
	printf(" 0123456789wo\n");


	return 0;
}


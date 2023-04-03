#include <stdio.h>

int main()
{
	int lower = 0,
		upper = 300,
		step = 20;

	printf("%6c %3c\n", 'c', 'F');
	float celsius = 0, fahr = 0;
	while (fahr <= upper)
	{
		celsius = (fahr - 32) * 5 / 9;
		printf("%6.1f %3.0f\n", celsius, fahr);
		fahr += step;
	}
	return 0;
}


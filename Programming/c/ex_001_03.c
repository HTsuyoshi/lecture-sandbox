#include <stdio.h>

int main()
{
	int lower = 0,
		upper = 300,
		step = 20;

	printf("%3c %6c\n", 'c', 'F');
	float celsius = 0, fahr = 0;
	while (fahr <= upper)
	{
		celsius = (fahr - 32) * 5 / 9;
		printf("%3.0f %6.1f\n", fahr, celsius);
		fahr += step;
	}
	return 0;
}


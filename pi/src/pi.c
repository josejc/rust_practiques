#include <stdio.h>
#include <stdlib.h>

double rand_u(); /* 0.0 to 1.0, unif. dist. */

int main() {

	double p_dins_cercle = 0;
	double x, y;
	unsigned long i;
	unsigned long n = 0;

	printf("Numero punts [0=1.000.000]:");
        scanf("%lu",&n);
	if (n==0) {
		n=1000000;
	}
	
	for (i=0;i<n;i++) {
		x = rand_u();	// x ϵ (0,1)
		y = rand_u();	// y ϵ (0,1)
		// x²+y² <= 1 -> A_circle = πr² -> A_circle/4 ^ r=1 => π/4
		// A_rect = 1*1 = 1
		// Only calculate points in a quarter of circle -> Rectangle 1*1
		if ((x*x)+(y*y) <= 1.00) {
			p_dins_cercle++;
		}
	}
	printf("punts: %lu, pi aprox. %0.15f\n", n, (p_dins_cercle/n)*4.0);
	return 0;
}

double rand_u() {
   return rand()/(double)RAND_MAX;
}

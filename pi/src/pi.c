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
		x = rand_u();
		y = rand_u();
		if (((x*x)-x+(y*y)-y) <= -1.0/4.0) {
			p_dins_cercle++;
		}
	}
	printf("punts: %lu, pi aprox. %0.15f\n", n, (p_dins_cercle/n)*4.0);
	return 0;
}

double rand_u() {
   return rand()/(double)RAND_MAX;
}

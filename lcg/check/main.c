#include <stdio.h>
#include "rand.h"

int main()
{
   int i;
	
   for (i=1; i<25; i++){
      printf("%f, ", rand(1));
   }
   printf("\n");
   return 0;
}


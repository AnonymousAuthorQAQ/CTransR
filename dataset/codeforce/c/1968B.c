#include <stdio.h>
#include <string.h>
#include <stdlib.h>


int main()
{
     int x, y, z, i, j, k, a, b, c, d, e, f, n, m, t;
     char sn[200001], sm[200001];
     
     scanf("%d", &t);
     while(t--)
     {
               scanf("%d %d %s %s", &n, &m, sn, sm);
               i = j = 0;
               while(i < n && j < m)
               {
                       if(sn[i] == sm[j])
                               i++;
                       j++;              
               }
               
               printf("%d\n", i);
     
     }
}


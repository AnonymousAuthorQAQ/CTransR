#include <stdio.h>
#include <string.h>
#include <stdlib.h>


#define  N        200000

int min(int a, int b) { return a < b ? a : b; }
long long maxl(long long a, long long b) { return a > b ? a : b; }

int main()
{
     int x, y, z, i, j, i_, j_, k, a, b, a_, b_, c, d, e, f, n, m, t, arr[N + 1], aa[N + 1];
     long long p1, p2, p1_, p2_;
     
     scanf("%d", &t);
     while(t--)
     {
               p1 = p2 = p1_ = p2_ = 0;
               i = i_ = j = j_ = -1;
               scanf("%d %d %d %d", &n, &k, &a, &b),  m = min(n, k),  a_ = a,  b_ = b;
               for(x = 1; x <= n; x++)
                       scanf("%d", &arr[x]);
               for(x = 1; x <= n; x++)
                       scanf("%d", &aa[x]);
               
               for(x = 1; x <= m; x++) {
                       p1 = maxl(p1, 1ll * aa[a] * (k - x + 1) + p1_);
                       p1_ += aa[a],  a = arr[a];
                       p2 = maxl(p2, 1ll * aa[b] * (k - x + 1) + p2_);
                       p2_ += aa[b],  b = arr[b];
               } 
                       
               printf(p1 > p2 ? "Bodya\n" : p1 < p2 ? "Sasha\n" : "Draw\n");                       
                               
     }
}


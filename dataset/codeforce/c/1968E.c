#include <stdio.h>

void solve(int n) {
    int t = n;
    printf("1 1\n");
    printf("2 1\n");
    t -= 2;
    if (t) {
        printf("%d %d\n", n, n);
        t--;
    }
    for (int i = 3; i < 3 + t; i++) {
        printf("1 %d\n", i);
    }
}

int main() {
    int t = 1;
    scanf("%d", &t);
    while (t--) {
        int n;
        scanf("%d", &n);
        solve(n);
    }
    return 0;
}


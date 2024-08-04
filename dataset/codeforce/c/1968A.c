#include <stdio.h>
int gcd(int a, int b) {
    if (b == 0)
        return a;
    return gcd(b, a % b);
}
int findY(int x) {
    return x - 1;
}
int main() {
    int t;
    scanf("%d", &t);
    while (t--) {
        int x;
        scanf("%d", &x);
        int y = findY(x);
        printf("%d\n", y);
    }

    return 0;
}


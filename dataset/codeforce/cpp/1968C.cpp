#include <iostream>
using namespace std;

#define N 500
#define X 500
#define A 1000000000

int x[N], a[N];

void proc() {
    int n, i, j;
    cin >> n;
    for (i = 1; i < n; ++i) cin >> x[i];
    a[0] = x[1] + 1;
    for (i = 1; i < n; ++i) {
        for (j = 0; j <= A; ++j) {
            a[i] = x[i] + j * a[i - 1];
            if ((i + 1 < n ? x[i + 1] : 1) < a[i] && a[i] != a[i - 1]) {
                break;
            }
        }
    }
    for (i = 0; i < n; ++i) cout << a[i] << " ";
    cout << "\n";
}

int main() {
    int t;
    cin >> t;
    while (t--) proc();
    return 0;
}


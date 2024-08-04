#include <bits/stdc++.h>
using namespace std;
#define ll long long
#define Youssef_Farghaly ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
#define el "\n"
ll x;
bool can(ll mid) {
    return gcd(x, mid) + mid ;
}
int main() {
    Youssef_Farghaly
        ll t;
    cin >> t;
    while (t--) {
        cin >> x;
        ll l = 1, r = x - 1;
        ll ans = 1;
        while (l <= r) {
            ll mid = (l + r) / 2;
            if (can(mid)) {
                ans = mid;
                l = mid + 1;
            }
            else {
                r = mid - 1;
            }
        }
        cout << ans << el;
    }
    return 0;
}

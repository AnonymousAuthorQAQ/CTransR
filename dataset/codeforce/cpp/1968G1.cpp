#include <bits/stdc++.h>
#include<ext/pb_ds/priority_queue.hpp>
#include <ext/pb_ds/assoc_container.hpp>
using namespace __gnu_pbds;
using namespace std;
typedef unsigned long long ull;
#define IO ios::sync_with_stdio(false), cin.tie(nullptr), cout.tie(nullptr);
#define All(a) a.begin(), a.end()
#define lb lower_bound
#define ub upper_bound
#define INF 0x3f3f3f3f3f3f3f3f
#define int long long
#define Z cout << "\n"
#define D(x) cerr << #x << ": " << (x) << "\n"
#define DV(v) cerr<<#v<<": ";for(int i=0;i<(v).size();i++)cerr<<((v)[i])<<",";cerr<<"\n"
typedef pair<int, int> PII;
void fix(int _z) { cout << fixed << setprecision(_z); }
template <class T>
using rbtree = tree<T, null_type, std::less<T>, rb_tree_tag, tree_order_statistics_node_update>;
template <class T>
using pheap = __gnu_pbds::priority_queue<T, less<T>, pairing_heap_tag>;
vector<int> z_function(string s) {
    int n = (int)s.length();
    vector<int> z(n);
    for (int i = 1, l = 0, r = 0; i < n; ++i) {
        if (i <= r && z[i - l] < r - i + 1) {
            z[i] = z[i - l];
        }
        else {
            z[i] = max(0LL, r - i + 1);
            while (i + z[i] < n && s[z[i]] == s[i + z[i]]) ++z[i];
        }
        if (i + z[i] - 1 > r) l = i, r = i + z[i] - 1;
    }
    z[0] = n;
    return z;
}
signed main() {
    IO;
    int t;
    cin >> t;
    while (t--) {
        int n, l, r;
        cin >> n >> l >> r;
        string s;
        cin >> s;
        int ls = 0, rs = n + 1;
        auto z = z_function(s);
        auto f = [&](int x)->bool {
            if (x > n / l)return false;
            string t;
            int cnt = 0;
            for (int i = x; i < s.size(); ) {
                if (z[i] >= x) {
                    cnt++;
                    i += x;
                }
                else i++;
            }
            return cnt >= l - 1;
            };
        while (ls + 1 < rs) {
            int mid = ls + rs >> 1;
            if (f(mid))ls = mid;
            else rs = mid;
        }
        cout << ls << "\n";
    }
    return 0;
}

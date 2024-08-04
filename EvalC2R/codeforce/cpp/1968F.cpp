#include <bits/stdc++.h>
using namespace std;
 
#define ll long long
#define pll pair<ll, ll>
#define pii pair<int, int>
#define fs first
#define sc second
#define pb push_back
 
void solve() {
    int n,q;
    cin>>n>>q;
    int arr[n];
    for (int i=0; i<n; i++) cin>>arr[i];
    pii x[n+1];
    int x2[n+1];
    x[0]={0,0};
    for (int i=1;i<=n;i++)x[i]={x[i-1].fs^arr[i-1],i};
    for (int i=0;i<=n;i++)x2[i]=x[i].fs;
    sort(x,x+n+1);
    
    while (q--){
        int l,r;
        cin>>l>>r;
        int c=x2[r]^x2[l-1];
        if (c==0){
            cout<<"YES\n";
            continue;
        }
        int i1=lower_bound(x,x+n+1,make_pair(c^x2[l-1],l))-x;
        
        if(i1==n+1||x[i1].sc>r||x[i1].fs!=(c^x2[l-1])){
            cout<<"NO\n";
            continue;
        }
        int i2=lower_bound(x,x+n+1,make_pair(x2[l-1],x[i1].sc+1))-x;
        
        if (i2==n+1||x[i2].sc>r||x[i2].fs!=x2[l-1]){
            cout<<"NO\n";
        } else cout<<"YES\n";
 
        
    }
}
 
int main() {   
    ios_base::sync_with_stdio(0); cin.tie(0); cout.tie(0);
 
    int t; cin>>t; while (t--) solve();
}

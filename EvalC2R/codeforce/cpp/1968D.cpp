#include <bits/stdc++.h>
using namespace std;

#define ll long long 
#define endl '\n'

ll calculate(vector<int> &p,vector<int> &a,int pos,int k){
    int n=p.size();
    ll mx=0,cur=0;
    vector<bool>vis(n);
    while(!vis[pos] and k>0){
        vis[pos]=true;
        mx=max(mx,cur+(1LL*k*a[pos]));
        cur+=a[pos];
        pos=p[pos];
        k--;
    }
    return mx;
}
signed main(){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t=1;cin >> t;
    while(t--){
        int n,k,ps1,ps2;cin >> n>>k>>ps1>>ps2;
        vector<int> p(n),a(n);
        for(int i=0;i<n;i++)cin >> p[i],--p[i];
        for(int i=0;i<n;i++)cin >> a[i];
        ll ans1=calculate(p,a,ps1-1,k);
        ll ans2=calculate(p,a,ps2-1,k);
        if(ans1==ans2){
            cout<<"Draw"<<endl;
        }
        else if(ans1>ans2)cout<<"Bodya"<<endl;
        else cout<<"Sasha"<<endl;
        }
    return 0;
}

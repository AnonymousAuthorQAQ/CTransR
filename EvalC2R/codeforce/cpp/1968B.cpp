#include <bits/stdc++.h>
using namespace std;
int main(){
	ios::sync_with_stdio(0);cin.tie(0);
	int t;cin>>t;
	while(t--){
		int x,k; cin>>x>>k;
		string a,b;cin>>a>>b;
		int ans=0;
		for(auto i:b){
			if(ans < a.size() && a[ans] == i){
				ans++;
			}
		}
		cout<<ans<<"\n";
	}
	return 0;
}

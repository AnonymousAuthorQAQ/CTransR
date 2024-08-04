#include <bits/stdc++.h>
using namespace std;
using ll=long long;
void solve(){	
	int n;
	cin>>n;
	int t=n;
	cout<<"1 1\n";
	cout<<"2 1\n";
	t-=2;
	if(t){
		cout<<n<<" "<<n<<'\n';
		t--;
	}
	for(int i=3;i<3+t;i++){
		cout<<"1 "<<i<<'\n';
	}
	return;
}
int main(){
	ios::sync_with_stdio(0);cin.tie(0),cout.tie(0);
	int t=1;
	cin>>t;
	while(t--){
		solve();
	}
	return 0;
}




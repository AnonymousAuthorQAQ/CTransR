#include<bits/stdc++.h>
const int N=2e5+3;
int n,x,y,z[N],p[N],t[N],ans[N];
char a[N];
int Find(int k){return k>n||p[k]==k?k:p[k]=Find(p[k]);}
int main(){
	int T;scanf("%d",&T);for(;T--;){
		scanf("%d%d%d%s",&n,&x,&y,a+1);
		for(int i=1;i<=n+1;i++)p[i]=i,t[i]=i,ans[i]=0,z[i]=0;
		z[1]=n;
		for(int i=2,j=0;i<=n;i++){
			if(j+z[j]>i)z[i]=std::min(z[i-j+1],j+z[j]-i);
			for(;a[i+z[i]]==a[1+z[i]];z[i]++);
			if(i+z[i]>j+z[j])j=i;
		}
		std::sort(t+1,t+1+n,[](const int&i,const int&j){return z[i]<z[j];});
		for(int i=1,k=1;i<=n;i++){
			int c=0;
			for(;k<=n&&z[t[k]]<i;k++)p[t[k]]=t[k]+1;
			for(int j=1;j<=n;j=Find(j+i))
				++c;
			ans[c]=i;
		}
		for(int i=n;i;i--)ans[i]=std::max(ans[i],ans[i+1]);
		for(int i=x;i<=y;i++)printf("%d ",ans[i]);puts("");
	}
	return 0;
}


#include<stdio.h>
#include<stdlib.h>
#include<time.h>
unsigned rand1(){return rand()*1u*rand();}
unsigned rand2(unsigned l, unsigned r){if(l==r)return l;return rand1()%(r-l)+l;}

#define N 200005

const int MOD = 1000000007;

char s[N+2];
int n,l,r,hash[N], BASE, base_pow[N+1];

int power(int a,int b)
{
    if(!b)return 1;
    int r=power(a,b/2);
    if(b&1)return r*1ll*r%MOD*a%MOD;
    return r*1ll*r%MOD;
}

int check(int k)
{
    int vo=0;
    for(int i=1;i+k-1<=n;)
        if ((hash[i+k-1] + MOD - hash[i-1]) % MOD == hash[k] * 1ll * base_pow[i-1] %MOD)
            ++vo, i+=k;
        else
            ++i;
    return vo;
}

int main()
{
    srand(time(0));
    BASE=rand2(200,MOD-1);

    int t;
    scanf("%d",&t);
    while(t--)
    {
        scanf("%d%d%d%s",&n,&l,&r,s+1);

        int bp=1;
        base_pow[0]=1;
        for(int i=1;i<=n;++i,bp=bp*1ll*BASE%MOD)
            base_pow[i]=bp*1ll*BASE%MOD,hash[i]=bp*1ll*s[i]%MOD,hash[i]=(hash[i]+hash[i-1])%MOD;

        int lower=0,upper=n+1,yy;
        while(upper-lower>1)
        {
            int mid=lower+(upper-lower)/2;
            if((yy=check(mid))>=l)lower=mid;
            else upper=mid;
        }
        printf("%d",lower);

        for(int i=l+1;i<=r;++i)
        {
            if(i>yy)
            {
                upper=lower+1;
                lower=0;
                while(upper-lower>1)
                {
                    int mid=lower+(upper-lower)/2;
                    if((yy=check(mid))>=i)lower=mid;
                    else upper=mid;
                }
            }
            printf("%d",lower);
        }

        putchar(10);
    }
}



#include <stdio.h>
#include <string.h>

#define N	200000
#define N_	(1 << 18)	/* N_ = pow2(ceil(log2(N))) */

int max(int a, int b) { return a > b ? a : b; }

int st[N_ * 2], n_;

void build(int *aa, int n) {
	int i;

	n_ = 1;
	while (n_ < n)
		n_ <<= 1;
	memset(st, 0, n_ * 2 * sizeof *st);
	for (i = 0; i < n; i++)
		st[n_ + i] = aa[i];
	for (i = n_ - 1; i > 0; i--)
		st[i] = max(st[i << 1 | 0], st[i << 1 | 1]);
}

int query(int l, int a) {
	int r = n_ - 1;

	for (l += n_, r += n_; l <= r; l >>= 1, r >>= 1)
		if ((l & 1) == 1) {
			if (st[l] >= a) {
				while (l < n_)
					l = st[l << 1 | 0] >= a ? l << 1 | 0 : l << 1 | 1;
				return l - n_;
			}
			l++;
		}
	return n_;
}

int main() {
	int t;

	scanf("%d", &t);
	while (t--) {
		static char cc[N + 1];
		static int zz[N], ans[N + 1];
		int n, kl, kr, i, l, r, p, k;

		scanf("%d%d%d%s", &n, &kl, &kr, cc);
		memset(zz, 0, n * sizeof *zz);
		for (i = 1, l = r = 0; i < n; i++)
			if (i + zz[i - l] < r)
				zz[i] = zz[i - l];
			else {
				l = i, r = max(r, i);
				while (r < n && cc[r] == cc[r - l])
					r++;
				zz[i] = r - l;
			}
		build(zz, n);
		memset(ans, 0, (n + 1) * sizeof *ans);
		for (p = 1; p <= n; p++) {
			k = 0;
			for (i = 0; i < n; i = query(i + p, p))
				k++;
			ans[k] = max(ans[k], p);
		}
		for (k = n - 1; k > 0; k--)
			ans[k] = max(ans[k], ans[k + 1]);
		for (k = kl; k <= kr; k++)
			printf("%d ", ans[k]);
		printf("\n");
	}
	return 0;
}


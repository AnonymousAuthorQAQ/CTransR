#include <stdio.h>
#include <sys/time.h>

#define N	200000

unsigned int Z;

int rand_() {
	return (Z *= 3) >> 1;
}

void srand_() {
	struct timeval tv;

	gettimeofday(&tv, NULL);
	Z = tv.tv_sec ^ tv.tv_usec | 1;
}

int aa[N + 1], n;

void sort(int *ii, int l, int r) {
	while (l < r) {
		int i = l, j = l, k = r, i_ = ii[l + rand_() % (r - l)], tmp;

		while (j < k) {
			int c = aa[ii[j]] != aa[i_] ? aa[ii[j]] - aa[i_] : ii[j] - i_;

			if (c == 0)
				j++;
			else if (c < 0) {
				tmp = ii[i], ii[i] = ii[j], ii[j] = tmp;
				i++, j++;
			} else {
				k--;
				tmp = ii[j], ii[j] = ii[k], ii[k] = tmp;
			}
		}
		sort(ii, l, i);
		l = k;
	}
}

int ii[N + 1];

int search(int a, int i) {
	int lower = -1, upper = n + 1;

	while (upper - lower > 1) {
		int h = (lower + upper) / 2;

		if (aa[ii[h]] < a || aa[ii[h]] == a && ii[h] < i)
			lower = h;
		else
			upper = h;
	}
	return lower;
}

int main() {
	int t;

	srand_();
	scanf("%d", &t);
	while (t--) {
		int q, i;

		scanf("%d%d", &n, &q);
		for (i = 1; i <= n; i++) {
			scanf("%d", &aa[i]);
			aa[i] ^= aa[i - 1];
		}
		for (i = 0; i <= n; i++)
			ii[i] = i;
		sort(ii, 0, n + 1);
		while (q--) {
			int l, r;

			scanf("%d%d", &l, &r), l--;
			printf(aa[l] == aa[r] || ii[search(aa[r], l) + 1] < ii[search(aa[l], r)] ? "YES\n" : "NO\n");
		}
	}
	return 0;
}


#include<bits/stdc++.h>
using namespace std;

int n = 40;
vector<vector<int>> g(n, vector<int>(n, -1));
bool check(int i, int j) {
	if (i < 0 || i >= n || j < 0 || j >= n) return false;
	return true;
}
long finder(int i, int j) {
	if (g[i][j] == 9) {
		return 1L;
	}

	long result = 0;
	if (check(i+1, j) && g[i][j] + 1 == g[i+1][j]) result += finder(i+1, j);
	if (check(i, j+1) && g[i][j] + 1 == g[i][j+1]) result += finder(i, j+1);
	if (check(i-1, j) && g[i][j] + 1 == g[i-1][j]) result += finder(i-1, j);
	if (check(i, j-1) && g[i][j] + 1 == g[i][j-1]) result += finder(i, j-1);
	return result;
}
int main() {
	char ch;
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; ++j) {
			cin >> ch;
			g[i][j] = ch - '0';
		}
	}
	long ans = 0;
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; ++j) {
			if (g[i][j] == 0) 
				ans += finder(i, j);
		}
	}
	cout << ans << '\n';
	return 0;
}

#include<bits/stdc++.h>
using namespace std;

int n = 40;
vector<vector<int>> g(n, vector<int>(n, -1));
bool check(int i, int j) {
	if (i < 0 || i >= n || j < 0 || j >= n) return false;
	return true;
}
void finder(int i, int j, set<pair<int, int>> &history) {
	if (g[i][j] == 9) {
		history.insert({i, j});
		return;
	}

	if (check(i+1, j) && g[i][j] + 1 == g[i+1][j]) finder(i+1, j, history);
	if (check(i, j+1) && g[i][j] + 1 == g[i][j+1]) finder(i, j+1, history);
	if (check(i-1, j) && g[i][j] + 1 == g[i-1][j]) finder(i-1, j, history);
	if (check(i, j-1) && g[i][j] + 1 == g[i][j-1]) finder(i, j-1, history);
}
int main() {
	char ch;
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; ++j) {
			cin >> ch;
			g[i][j] = ch - '0';
		}
	}
	set<pair<int, int>> history;
	int ans = 0;
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; ++j) {
			history.clear();
			if (g[i][j] == 0) finder(i, j, history);
			ans += history.size();
		}
	}
	cout << ans << '\n';
	return 0;
}

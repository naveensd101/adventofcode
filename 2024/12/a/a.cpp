#include<bits/stdc++.h>
using namespace std;

int N = 140;
vector<vector<char>> a(N, vector<char>(N, '#'));
vector<vector<bool>> visitedA(N, vector<bool>(N, false));
vector<vector<bool>> visitedP(N, vector<bool>(N, false));

bool isItSafe(int i, int j) {
	if(i < 0 || i > N - 1 || j < 0 || j > N - 1) return false;
	return true;
}

int area(char ch, int i, int j) {
	if (!isItSafe(i, j)) return 0;
	if (visitedA[i][j]) return 0;
	if (ch != a[i][j]) return 0;
	visitedA[i][j] = true;
	return 1
		+ area(ch, i+1, j)
		+ area(ch, i, j+1)
		+ area(ch, i-1, j)
		+ area(ch, i, j-1);
}

int perimeter(char ch, int i, int j) {
	if (!isItSafe(i, j)) return 0;
	if (visitedP[i][j]) return 0;
	if (ch != a[i][j]) return 0;
	visitedP[i][j] = true;

	int ans = 0;
	if (!isItSafe(i+1, j) || ch != a[i+1][j]) ans++;
	else ans += perimeter(ch, i+1, j);
	if (!isItSafe(i, j+1) || ch != a[i][j+1]) ans++;
	else ans += perimeter(ch, i, j+1);
	if (!isItSafe(i-1, j) || ch != a[i-1][j]) ans++;
	else ans += perimeter(ch, i-1, j);
	if (!isItSafe(i, j-1) || ch != a[i][j-1]) ans++;
	else ans += perimeter(ch, i, j-1);

	return ans;
}

int main() {
	for (int i = 0; i < N; ++i) {
		for (int j = 0; j < N; ++j) {
			cin >> a[i][j];
		}
	}
	// for (int i = 0; i < N; ++i) {
	// 	for (int j = 0; j < N; ++j) {
	// 		cout << a[i][j] << ' ';
	// 	}
	// 	cout << '\n';
	// }
	int ans = 0;
	for (int i = 0; i < N; ++i) {
		for (int j = 0; j < N; ++j) {
			if(!visitedA[i][j] and !visitedP[i][j]) {
				ans += area(a[i][j], i, j) * perimeter(a[i][j], i, j);
				// cout << a[i][j]
				// 	<< ": "
				// 	<< area(a[i][j], i, j)
				// 	<< " "
				// 	<< perimeter(a[i][j], i, j)
				// 	<< '\n';
			}
		}
	}
	cout << ans << endl;
	return 0;
}

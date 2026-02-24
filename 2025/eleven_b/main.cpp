#include<bits/stdc++.h>
using namespace std;

map<string, vector<string>> graph;

set<string> visited;
map<string, long long> distances;
long long dfs(string from, string dest) {
	if (from == dest) return 1;
	if (visited.count(from)) return distances[from];

	long long waysToReachDest = 0;
	for (string child: graph[from]) {
		waysToReachDest += dfs(child, dest);
	}
	distances[from] = waysToReachDest;
	visited.insert(from);
	return waysToReachDest;
}

int main() {
	string line;
	while(getline(cin, line)) {
		stringstream ss(line);
		string left, word;
		ss >> left;
		left = left.substr(0, left.size()-1);
		while (ss >> word) {
			graph[left].push_back(word);
		}
	}
	string start = "dac", end = "fft";
	cout << "dist between " << start << " and " << end << " " <<  dfs(start, end) << '\n';

	return 0;
}

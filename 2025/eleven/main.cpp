#include<bits/stdc++.h>
using namespace std;

map<string, vector<string>> graph;

set<vector<string>> paths;
void dfs(string node, vector<string> parents) {
	for (string child: graph[node]) {
		vector<string> new_parent = parents;
		new_parent.push_back(child);
		if (child == "out") paths.insert(new_parent);
		dfs(child, new_parent);
	}
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
	dfs("svr", {"svr"});

	for (vector<string> path: paths) {
		for (string x: path) {
			cout << x << ' ';
		}
		cout << '\n';
	}
	cout << paths.size() << '\n';

	return 0;
}

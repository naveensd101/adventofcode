#include<bits/stdc++.h>

long num_of_digits(long num) {
	if (num < 10) return 1;
	return 1 + num_of_digits((num - num % 10) / 10);
}
std::unordered_map<std::pair<long, long>, long, boost::hash_combine> table;
long dfs(long val, long depth_to_go) {
	if (table.find(std::make_pair(val, depth_to_go)) != table.end()) {
		return table[std::make_pair(val, depth_to_go)];
	}
	if (depth_to_go == 0) {
		table[std::make_pair(val, depth_to_go)]  = 1;
		return 1;
	}
	long ans = 0;
	if (val == 0) {
		ans += dfs(1, depth_to_go - 1);
	} else if (num_of_digits(val) % 2 == 0) {
			std::string sval = std::to_string(val);
			std::string left = "", right = "";
			for (long j = 0; 2 * j < sval.size(); ++j) {
				left += sval[j];
				right += sval[sval.size() / 2 + j];
			}
			ans += dfs(std::stoi(left), depth_to_go - 1);
			ans += dfs(std::stoi(right), depth_to_go - 1);
	} else {
		ans += dfs(2024 * val, depth_to_go - 1);
	}
	table[std::make_pair(val, depth_to_go)] = ans;
	return ans;
}
void operate(list<long> &a) {
	for (auto i = a.begin(); i != a.end(); ++i) {
		if (*i == 0) {
			*i = 1;
		} else if (num_of_digits(*i) % 2 == 0) {
			string val = to_string(*i);
			string left = "", right = "";
			for (long j = 0; 2 * j < val.size(); ++j) {
				left += val[j];
				right += val[val.size() / 2 + j];
			}
			*i = stoi(right);
			a.insert(i, stoi(left));
		} else {
			*i *= 2024;
		}
	}
}
int main() {
	list<long> a = {125, 17};
	list<long> b = {41078, 18, 7, 0, 4785508, 535256, 8154, 447};
	list<long> c = {0};
	long ans = 0;
	for (long x: a) {
		ans += dfs(x, 6);
	}
	std::cout << ans;
	return 0;
}

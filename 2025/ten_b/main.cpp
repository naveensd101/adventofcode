#include<bits/stdc++.h>
using namespace std;

vector<vector<int>> buttons;
vector<int> bulb_joltage_req;

int find_sum_except_i_for_bulb_id(int bulb_id, int ignore_idx, vector<int> &state) {
	int ans = 0;
	for (int i = 0; i < buttons.size(); ++i) {
		if (i == ignore_idx) continue;

		for (int j = 0; j < buttons[i].size(); ++j) {
			if (bulb_id == buttons[i][j]) {
				if (state[i] == -1) {
					return -1;
				} else {
					ans += state[i];
				}
			}
		}
	}
	return ans;
}

bool normalise(vector<int> &state) {
	bool did_state_change = false;
	for(int i = 0; i < state.size(); ++i) {
		int click = state[i];
		if(click == -1) {
			for(int bulb_id: buttons[i]) {
				int sum = find_sum_except_i_for_bulb_id(bulb_id, i, state);
				if (sum != -1 && bulb_joltage_req[bulb_id] - sum > -1) {
					state[i] = bulb_joltage_req[bulb_id] - sum;
					did_state_change = true;
					break;
				}
			}
		}
	}
	return did_state_change;
}

bool is_state_a_final_state(vector<int> &state) {
	vector<int> current_joltage = vector<int>(bulb_joltage_req.size(), 0);
	for (int i = 0; i < state.size(); ++i) {
		if (state[i] < 0) return false;
		for (int bulb_id: buttons[i]) {
			current_joltage[bulb_id] += state[i];
		}
	}
	return current_joltage == bulb_joltage_req;
}

vector<int> add(vector<int> state, int button_idx) {
	state[button_idx]++;
	return state;
}

void prt1d(vector<int> arr) {
	for (int x: arr) cout << x << ' ';
	cout << '\n';
}
void prt2d(vector<vector<int>> arr) {
	for (vector<int> lst: arr) {
		prt1d(lst);
	}
}

bool is_state_valid(vector<int> &state) {
	// 1. Cant be smaller than -1
	// 2. ith state's click value should not exceed the joltage req of its corresponding button value
	for (int i = 0; i < state.size(); ++i) {
		if (state[i] < -1) return false;

		int clicks = state[i];
		for (int bulb_id: buttons[i]) {
			if (clicks > bulb_joltage_req[bulb_id]) return false;
		}
	}
	return true;
}

int ans = INT_MAX;
set<vector<int>> visited;
void dfs(vector<int> &state) {
	if (visited.count(state)) return;
	else visited.insert(state);
	if (is_state_a_final_state(state)) {
		int sum = 0;
		for (int x: state) sum+=x;
		if (sum < ans) ans = sum;
	}

	for (int i = 0; i < buttons.size(); ++i) {
		vector<int> new_state = add(state, i);
		if (!is_state_valid(new_state)) continue;
		while(normalise(new_state));
		if (!is_state_valid(new_state)) continue;
		dfs(new_state);
	}
}

void line_reader() {
	string word;
	vector<vector<int>> buttons_local;
	vector<int> req;
	while(true) {
		cin >> word;
		if (word[0] == '[') continue;
		else if (word[0] == '(') {
			string csv = word.substr(1, word.size()-2);
			vector<int> tmp = vector<int>();
			string buffer = "";
			for (int i = 0; i < csv.size(); ++i) {
				if (csv[i] >= '0' && csv[i] <= '9') buffer += csv[i];
				else {
					tmp.push_back(stoi(buffer));
					buffer = "";
				}
			}
			buttons_local.push_back(tmp);
			tmp = vector<int>();
		}

		if (word[0] == '{') break;
	}
	prt2d(buttons_local);
}

int main() {
	//buttons = {
	//	{3},
	//	{1, 3},
	//	{2},
	//	{2, 3},
	//	{0, 2},
	//	{0, 1}
	//};
	//bulb_joltage_req = {3, 5, 4, 7};
	//vector<int> state = vector<int>(buttons.size(), -1);
	//dfs(state);

	int T;
	cin >> T;
	string str;
	while(T--) {
		line_reader();
	}
	cout << "ans = " << ans << '\n';
	return 0;
}

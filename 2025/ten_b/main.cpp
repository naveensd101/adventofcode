#include<iostream>
#include<vector>
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
			current_joltage[i] += state[i];
		}
	}
	return current_joltage == bulb_joltage_req;
}

int main() {
	buttons = {
		{3},
		{1, 3},
		{2},
		{2, 3},
		{0, 2},
		{0, 1}
	};
	bulb_joltage_req = {3, 5, 4, 7};
	vector<int> state = vector<int>(buttons.size(), -1);
	state = {1, 3, 0, 3, 1, 2};
	cout << is_state_a_final_state(state);


	return 0;
}
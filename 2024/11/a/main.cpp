#include<bits/stdc++.h>
using namespace std;

long num_of_digits(long num) {
	if (num < 10) return 1;
	return 1 + num_of_digits((num - num % 10) / 10);
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
	for (int i = 0; i < 75; ++i) {
		operate(c);
		for (long x: c) {
			cerr << x << ' ';
		}
		cerr << '\n';
	}
	cout << b.size();
	return 0;
}

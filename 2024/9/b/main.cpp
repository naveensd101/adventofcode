#include<bits/stdc++.h>
using namespace std;

class Data {
	public:
	int ID; // ID = -1 means its a '.'
	int size;

	Data(int _ID, int _size) {
		ID = _ID;
		size = _size;
	}
};
int main() {
	list<Data> hard_disk;
	int N = 19999;
	char ch;
	for (int i = 0, id = 0; i < N; i++) {
		cin >> ch;
		if (i % 2 == 0 && ch != '0') {
			hard_disk.push_back(Data(id++, (ch - '0')));
		} else if (ch != '0') {
			hard_disk.push_back(Data(-1, (ch - '0')));
		}
	}
	//cout << '\n';
	//for (auto iter = hard_disk.begin(); iter != hard_disk.end(); iter++) {
	//	for (int i = 0; i < iter -> size; i++) {
	//		if (iter -> ID == -1) {
	//			cout << '.';
	//		} else {
	//			cout << iter -> ID;
	//		}
	//	}
	//}
	//cout << '\n';
	for (auto now = --(hard_disk.end()); now != --(hard_disk.begin()); now--) {
		for (auto cleaner = hard_disk.begin(); now -> ID != -1 && cleaner != now; cleaner++) {
			if (cleaner -> ID == -1 && cleaner -> size >= now -> size) {
				int new_clean_size = cleaner -> size - now -> size;
				if (new_clean_size > 0) {
					hard_disk.insert(cleaner, 1, Data(now -> ID, now -> size));
					cleaner -> size = new_clean_size;
					now -> ID = -1;
				} else {
					cleaner -> ID = now -> ID;
					now -> ID = -1;
				}
				break;
			}
		}
		//for (auto iter = hard_disk.begin(); iter != hard_disk.end(); iter++) {
		//	for (int i = 0; i < iter -> size; i++) {
		//		if (iter -> ID == -1) {
		//			cout << '.';
		//		} else {
		//			cout << iter -> ID;
		//		}
		//	}
		//}
		//cout << '\n';
	}
	long ans = 0;
	long place = 0;
	for (auto iter = hard_disk.begin(); iter != hard_disk.end(); iter++) {
		for (int i = 0; i < iter -> size; i++) {
			if (iter -> ID != -1) {
				ans += (long)iter -> ID * place;
			}
			place++;
		}
	}
	cout << "ans = " << ans << '\n';
	return 0;
}

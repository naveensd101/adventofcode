#include<string>
#include<sstream>
#include<vector>
#include<iostream>

using namespace std;
int main() {
    for (int id = 1; id < 6; id++) {
        string game;
        bool isGamePossible = true;
        getline(cin, game);
        stringstream s(game);

        string word;
        s >> word >> word >> word;
        
        if (word[0] <= '9' and word[0] >= '0') {
            int num = stoi(word);
            s >> word;
            int red = 0, green = 0, blue = 0;
            if(word.compare("red")) {
                red = num;
            } else if(word.compare("green")) {
                green = num;
            } else if(word.compare("blue")) {
                blue = num;
            }
        }
        
    }
    return 0;
}

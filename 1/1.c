#include<stdio.h>

int len(char *str) {
    int i = 0;
    while(*str != '\0') i++, str++;
    return i;
}
int main(void) {
    char word[100];
    int sum = 0;
    while( scanf("%s", word) != EOF ) {
        int f, s;
        for(int i = 0; ; i++) {
            if(word[i] <= '9' && word[i] >= '0') {
                f = word[i] - '0';
                break;
            }
        }
        for(int i = len(word); ; i--) {
            if(word[i] <= '9' && word[i] >= '0') {
                s = word[i] - '0';
                break;
            }
        }
        int num = f * 10 + s;
        printf("%d\n", num);
        sum += num;
    }
    printf("su = %d\n", sum);
    return 0;
}

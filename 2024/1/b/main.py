N = 1000
def main():
    lst = []
    dic = {}
    for i in range(N):
        a, b = input().split()
        a = int(a)
        b = int(b)
        lst.append(a)
        if b in dic:
            dic[b] += 1
        else:
            dic[b] = 1

    ans = 0

    for i in range(N):
        if lst[i] in dic:
            ans += lst[i] * dic[lst[i]]
    print(ans)
main()

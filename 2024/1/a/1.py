def main():
    lst1 = []
    lst2 = []
    for i in range(1000):
        a, b = input().split()
        lst1.append(int(a))
        lst2.append(int(b))

    lst1.sort()
    lst2.sort()

    ans = 0
    for i in range(1000):
        ans += abs(lst1[i] - lst2[i])
    print(ans)

main()

def op(lst):
    n = len(lst)
    for i in range(n):
        if lst[i] == 0:
            lst[i] = 1
        elif len(str(lst[i])) % 2 == 0:
            a = len(str(lst[i]))
            left = str(lst[i])[:a//2]
            right = str(lst[i])[a//2:]

            lst[i] = int(left)
            lst.append(int(right))
        else:
            lst[i] *= 2024
table = {}
def dfs(elem, depth):
    if (elem, depth) in table:
        return table[(elem, depth)]
    if depth == 0:
        return 1
    ans = 0
    if elem == 0:
        ans = dfs(1, depth - 1)
    elif len(str(elem)) % 2 == 0:
        a = len(str(elem))
        left = str(elem)[:a//2]
        right = str(elem)[a//2:]

        ans = dfs(int(left), depth - 1) + dfs(int(right), depth - 1)
    else:
        ans = dfs(elem * 2024, depth - 1)
    table[(elem, depth)] = ans
    return ans

lst = [125, 17]
lst = [41078, 18, 7, 0, 4785508, 535256, 8154, 447]

ans = 0
for val in lst:
    ans += dfs(val, 75)
print(ans)

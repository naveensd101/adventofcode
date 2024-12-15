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

lst = [125, 17]
for i in range(22):
    print(i)
    op(lst)
print(len(lst))


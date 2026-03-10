n = int(input())
count = 0
for _ in range(n):
    line = input()
    split_line = line.split(" ")
    tree_area = split_line[0][:-1].split("x")
    available = int(tree_area[0]) // 3 * int(tree_area[1]) // 3
    needed = sum([int(x) for x in split_line[1:]])
    if needed <= available:
        count+=1
print(count)
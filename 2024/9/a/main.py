line = [int(ch) for ch in input()]
print(line)
print(len(line))

hard_disk = []
id_number = 0
i = 0
while(i < len(line)):
    if i % 2 == 0:
        hard_disk += [id_number] * line[i]
        id_number += 1
        i += 1
    else:
        hard_disk += ['.'] * line[i]
        i += 1
print(hard_disk)
i, j = 0, len(hard_disk) - 1

print(f"i = {i} {hard_disk[i]}, j = {j} {hard_disk[j]}")
while(i < j):
    if type(hard_disk[i]) != str:
        i += 1
        continue
    if type(hard_disk[j]) != int:
        j -= 1
        continue
    hard_disk[i], hard_disk[j] = hard_disk[j], hard_disk[i]

ans = 0
for i, val in enumerate(hard_disk):
    if val == '.': break
    ans += i * hard_disk[i]
print(ans)

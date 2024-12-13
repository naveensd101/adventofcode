line = [int(ch) for ch in input()]
print(line)
print(len(line))

hard_disk = []
id_number = 0
i = 0
while(i < len(line)):
    if i % 2 == 0:
        if line[i] != 0: hard_disk.append([id_number, line[i]])
        id_number += 1
        i += 1
    else:
        if line[i] != 0: hard_disk.append(['.', line[i]])
        i += 1
print(hard_disk)

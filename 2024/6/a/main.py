def printMap(map):
    for i, line in enumerate(map):
        print(f"{i} : {line}")
def findGirl(map):
    for i, line in enumerate(map):
        for j, ch in enumerate(line):
            if ch == '^': return i, j
def isGirlAlive(i, j, N):
    if i < 0: return False
    if j < 0: return False
    if i >= N: return False
    if j >= N: return False
    return True
def whereShouldGirlMoveTo(i, j, ch):
    if ch == '^': return i-1, j
    if ch == 'v': return i+1, j
    if ch == '>': return i, j+1
    if ch == '<': return i, j-1

# N = 10
N = 130
map = [input() for _ in range(N)]
map = [list(line) for line in map]

i, j = findGirl(map)
while isGirlAlive(i, j, N):
    #printMap(map)
    I, J = whereShouldGirlMoveTo(i, j, map[i][j])
    if not(isGirlAlive(I, J, N)):
        map[i][j] = 'x'
        break
    
    if map[I][J] == '#':
        if map[i][j] == '^': map[i][j] = '>'
        elif map[i][j] == '>': map[i][j] = 'v'
        elif map[i][j] == 'v': map[i][j] = '<'
        elif map[i][j] == '<': map[i][j] = '^'
    else:
        map[I][J] = map[i][j]
        map[i][j] = 'x'
        i, j = I, J

ans = 0
for lst in map:
    for ch in lst:
        if ch == 'x':
            ans += 1
print(f"ans = {ans}")

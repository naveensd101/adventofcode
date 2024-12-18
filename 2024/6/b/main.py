def printMap(map):
    for i, line in enumerate(map):
        print(f"{i} : {line}")
def findGirl(map):
    for i, line in enumerate(map):
        for j, ch in enumerate(line):
            if ch == '^': return i, j
def printDic(dic):
    for key in dic:
        print(key, end=" : ")
        print(dic[key])
def isGirlAlive(i, j, N):
    if i < 0 : return False
    if j < 0 : return False
    if i >= N: return False
    if j >= N: return False
    return True
def isOutOfBounds(i, j, N):
    return not(isGirlAlive(i, j, N))
def whereShouldGirlMoveTo(i, j, ch):
    if ch == '^': return i-1, j
    if ch == 'v': return i+1, j
    if ch == '>': return i, j+1
    if ch == '<': return i, j-1

    print("you fucked up inside whereShouldGirlMoveTo : {i}, {j}")
def rotateArrow(ch):
    if ch == '^': return '>'
    if ch == '>': return 'v'
    if ch == 'v': return '<'
    if ch == '<': return '^'
def getHistory(map, N):
    from collections import defaultdict
    history = defaultdict(list)
    i, j = findGirl(map)
    while isGirlAlive(i, j, N):
        I, J = whereShouldGirlMoveTo(i, j, map[i][j])
        if not(isGirlAlive(I, J, N)):
            return history

        if map[i][j] in ['^', '>', 'v', '<']:
            history[(i,j)].append(map[i][j])
        if map[I][J] == '#':
            tmp = map[i][j]
            map[i][j] = rotateArrow(map[i][j])
            oi, oj = i, j
            i, j = whereShouldGirlMoveTo(i, j, map[i][j])
            map[i][j] = map[oi][oj]
            map[oi][oj] = tmp
        else:
            map[I][J] = map[i][j]
            i, j = I, J
def willThisPositionMakeALoop(_i, _j, map, N):
    history = {}
    if map[_i][_j] == '.': map[_i][_j] = '#'
    else: return False
    print(f"_i = {_i} _j = {_j}")

    i, j = findGirl(map)
    while isGirlAlive(i, j, N):
        if (i,j) in history:
            if history[(i,j)] == map[i][j]:
                printMap(map)
                return True

        I, J = whereShouldGirlMoveTo(i, j, map[i][j])
        if not(isGirlAlive(I, J, N)):
            return False

        history[(i,j)] = map[i][j]
        if map[I][J] == '#':
            tmp = map[i][j]
            map[i][j] = rotateArrow(map[i][j])
            oi, oj = i, j
            i, j = whereShouldGirlMoveTo(i, j, map[i][j])
            map[i][j] = map[oi][oj]
            map[oi][oj] = tmp
        else:
            map[I][J] = map[i][j]
            i, j = I, J

N = 10
# N = 130
map = [input() for _ in range(N)]
map = [list(line) for line in map]
history = getHistory(map, N)
printDic(history)

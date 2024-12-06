
def isInBounds(i, j, N):
    if i < 0 : return False
    if j < 0 : return False
    if i >= N: return False
    if j >= N: return False
    return True

def whereShouldGirlMoveTo(i, j, ch):
    if ch == '^': return i-1, j
    if ch == 'v': return i+1, j
    if ch == '>': return i, j+1
    if ch == '<': return i, j-1

def turnRight(ch):
    if ch == '^': return '>'
    if ch == '>': return 'v'
    if ch == 'v': return '<'
    if ch == '<': return '^'

def findGirl(maP):
    for i, line in enumerate(maP):
        for j, ch in enumerate(line):
            if ch == '^': return i, j

def simulate(i, j, maP, N):
    count = 0
    while isInBounds(i, j, N):
        I, J = whereShouldGirlMoveTo(i, j, maP[i][j])
        if count > N*N:
            return True
        if not(isInBounds(I, J, N)):
            return False
        
        if maP[I][J] == '#':
            maP[i][j] = turnRight(maP[i][j])
        else:
            maP[I][J] = maP[i][j]
            i, j = I, J
            count += 1


N = 130
maP = [input() for _ in range(N)]
maP = [list(line) for line in maP]
home_i, home_j = findGirl(maP)
from copy import deepcopy
ans = 0
for i in range(N):
    for j in range(N):
        print(i, j)
        new_map = deepcopy(maP)
        if new_map[i][j] == '.':
            new_map[i][j] = '#'
            if simulate(home_i, home_j, new_map, N):
                ans += 1
print(ans)
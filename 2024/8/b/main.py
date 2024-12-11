def printer(cartesian_plane):
    n = len(cartesian_plane)
    for i in range(n-1, -1, -1):
        print(cartesian_plane[i])

def lstPrinter(lst):
    for elem in lst:
        print(elem)

def findAllCh(ch, cartesian_plane):
    result = []
    for y, line in enumerate(cartesian_plane):
        for x, value in enumerate(line):
            if value == ch: result.append((x, y))
    return result

def nc2(lst):
    # takes in a list of tupples
    # return all the possibilities of choosing 2 of them
    result = []
    for i in range(len(lst)):
        for j in range(i+1, len(lst)):
            result.append((lst[i], lst[j]))
    return(result)

def x2x1Points(lst):
    # takes in list of tupples
    # each tuple has got 2 tuples in them which are cartesian points
    result = []
    for line in lst:
        A = line[0]
        B = line[1]

        # inside left
        a = (B[0] + 2 * A[0]) / 3
        b = (B[1] + 2 * A[1]) / 3
        if isinstance(a, int) and isinstance(b, int):
            result.append((a, b))

        # inside right
        a = (2 * B[0] + A[0]) / 3
        b = (2 * B[1] + A[1]) / 3
        if isinstance(a, int) and isinstance(b, int):
            result.append((a, b))

        # left
        a = 2 * A[0] - B[0]
        b = 2 * A[1] - B[1]
        result.append((a, b))

        # right
        a = 2 * B[0] - A[0]
        b = 2 * B[1] - A[1]
        result.append((a, b))
    return result

def isLegal(pt, N):
    if pt[0] >= 0 and pt[0] < N and pt[1] >= 0 and pt[1] < N: return True
    return False

def slopeEquate(tupe, pt):
    A = tupe[0]
    B = tupe[1]
    if (B[1] - A[1]) * (pt[0] - A[0]) == (pt[1] - A[1]) * (B[0] - A[0]):
        return True
    return False
    

N = 50
cartesian_plane = [input() for _ in range(N)]
cartesian_plane.reverse()

printer(cartesian_plane)
from collections import defaultdict
towers = defaultdict(list)
for y, line in enumerate(cartesian_plane):
    for x, ch in enumerate(line):
        if ch.isalnum():
            towers[ch].append((x, y))

result = [['.']*N for _ in range(N)]
for ch in towers:
    print(ch)
    print(towers[ch])
    lines = nc2(towers[ch])
    for line in lines:
        print(line)
        for y in range(N):
            for x in range(N):
                if slopeEquate(line, (x,y)):
                    print((x,y))
                    result[x][y] = '#'

ans = 0
for line in result:
    for ch in line:
        if ch == '#':
            ans += 1
printer(result)
print(ans)

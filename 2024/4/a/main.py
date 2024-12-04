N = 140
ar = []

def isIt(i, j, ch):
    if i < 0: return False
    if j < 0: return False
    if i >= N: return False
    if j >= N: return False

    if ar[i][j] == ch: return True
    else: return False
def check(i, j):
    ans = 0
    # check right
    if isIt(i, j, 'X') and isIt(i, j+1, 'M') and isIt(i, j+2, 'A') and isIt(i, j+3, 'S'): ans += 1
    # check right down diag
    if isIt(i, j, 'X') and isIt(i+1, j+1, 'M') and isIt(i+2, j+2, 'A') and isIt(i+3, j+3, 'S'): ans += 1
    # check down
    if isIt(i, j, 'X') and isIt(i+1, j, 'M') and isIt(i+2, j, 'A') and isIt(i+3, j, 'S'): ans += 1
    # check down left diag
    if isIt(i, j, 'X') and isIt(i+1, j-1, 'M') and isIt(i+2, j-2, 'A') and isIt(i+3, j-3, 'S'): ans += 1
    # check left
    if isIt(i, j, 'X') and isIt(i, j-1, 'M') and isIt(i, j-2, 'A') and isIt(i, j-3, 'S'): ans += 1
    # check up left
    if isIt(i, j, 'X') and isIt(i-1, j-1, 'M') and isIt(i-2, j-2, 'A') and isIt(i-3, j-3, 'S'): ans += 1
    # check up
    if isIt(i, j, 'X') and isIt(i-1, j, 'M') and isIt(i-2, j, 'A') and isIt(i-3, j, 'S'): ans += 1
    # check up right
    if isIt(i, j, 'X') and isIt(i-1, j+1, 'M') and isIt(i-2, j+2, 'A') and isIt(i-3, j+3, 'S'): ans += 1
    
    return ans
    
def main():
    for _ in range(N):
        ar.append(input())

    ans = 0
    for i in range(N):
        for j in range(N):
            ans += check(i, j)
    print(ans)

main()

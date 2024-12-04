N = 140
ar = [input() for _ in range(N)]

def check(i, j):
    # check small square
    diag_1 = ar[i][j] + ar[i+1][j+1] + ar[i+2][j+2]
    diag_2 = ar[i+2][j] + ar[i+1][j+1] + ar[i][j+2]
    if not(diag_1 == "MAS" or diag_1 == "SAM"): return False
    if not(diag_2 == "MAS" or diag_2 == "SAM"): return False
    return True

def main():
    ans = 0
    #print(ar)
    for i in range(N-2):
        for j in range(N-2):
            if check(i, j) :
                #print(f"({i}, {j})")
                ans += 1
    print(ans)

main()

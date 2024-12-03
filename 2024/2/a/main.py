def is_lst_good(lst):
    nature = lst[1] - lst[0]
    for i in range(1, len(lst)):
        if nature * (lst[i] - lst[i-1]) < 0:
            #print(f"0i = {i}")
            return False
        if abs(lst[i] - lst[i-1]) > 3:
            #print(f"1i = {i}")
            return False
        if abs(lst[i] - lst[i-1]) == 0:
            #print(f"2i = {i}")
            return False
    return True

def main():
    N = 1000
    ans = 0
    for i in range(N):
        line = input()
        lst = [ int(elm) for elm in line.split() ]
        #print(lst)
        if is_lst_good(lst):
            ans += 1
    print(ans)
main()

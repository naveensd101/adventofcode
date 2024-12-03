def is_lst_good(lst):
    nature = lst[1] - lst[0]
    for i in range(1, len(lst)):
        if nature * (lst[i] - lst[i-1]) < 0:
            return False
        if abs(lst[i] - lst[i-1]) > 3:
            return False
        if abs(lst[i] - lst[i-1]) == 0:
            return False
    return True

def perm(lst):
    mega_lst = []
    for i in range(len(lst)):
        tmp = [lst[x] for x in range(len(lst)) if x != i]
        mega_lst.append(tmp)
    return mega_lst

def main():
    N = 1000
    ans = 0
    for i in range(N):
        line = input()
        lst = [ int(elm) for elm in line.split() ]
        for x in perm(lst):
            if is_lst_good(x):
                ans += 1
                break
    print(ans)
main()

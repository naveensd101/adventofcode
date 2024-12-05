N = 1176 # number of conditions
M = 205  # number of lists
conditions = [input() for _ in range(N)]
conditions = [list(map(int, con.split('|'))) for con in conditions]
right_dic = {}
for con in conditions:
    if con[0] in right_dic:
        right_dic[con[0]].append(con[1])
    else:
        right_dic[con[0]] = [con[1]]
input() # empty line
lists = [input() for _ in range(M)]
lists = [list(map(int, lst.split(','))) for lst in lists]
print(lists)
print(right_dic)

def g(val, dic):
    if val in dic:
        return dic[val]
    return 100000
def f(lst_index_dic, lst):
    # check if this lst is ok with conditions
    # if yes return middle element
    # else return 0
    for elem in lst:
        right_array = []
        if elem in right_dic:
            right_array = right_dic[elem]
        for val in right_array:
            idx = g(val, lst_index_dic)
            if idx < lst_index_dic[elem]:
                return 0
    return lst[len(lst) // 2]

def main():
    ans = 0
    for lst in lists:
        lst_index_dic = {}
        for i,val in enumerate(lst):
            lst_index_dic[val] = i
        ans += f(lst_index_dic, lst)
    print(ans)



main()

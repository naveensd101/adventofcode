def dictPrint(dic):
    for key in dic:
        print(key, end=": ")
        print(dic[key])
def listPrint(lst):
    for l in lst:
        print(l)
def process(conditionsDic, lstDic, lst):
    i = 0
    globalDidWeChange = False
    while i < len(lst):
        number = lst[i]
        conditions = []
        if number in conditionsDic:
            conditions = conditionsDic[number]
        didWeChange = False
        for num in conditions:
            if num in lstDic and lstDic[num] < lstDic[number]:
                didWeChange = True
                globalDidWeChange = True
                lstDic[num], lstDic[number] = lstDic[number], lstDic[num]
                lst[lstDic[num]], lst[lstDic[number]] = lst[lstDic[number]], lst[lstDic[num]]
                i = min(lstDic[num], lstDic[number])
                # print(f"i = {i}")
                # print(lst)
        if not(didWeChange):
            i += 1
    if globalDidWeChange: 
        # print(lst[len(lst) // 2])

        return lst[len(lst) // 2]
    else:
        # print(0)
        return 0

# N = 21
# M = 6
N = 1176
M = 205

conditionsList = [input() for _ in range(N)]
input()
lists = [input() for _ in range(M)]
lists = [list(map(int, lst.split(','))) for lst in lists]

conditionsList = [list(map(int, val.split('|'))) for val in conditionsList]

conditionsDic = {}
for conditionPair in conditionsList:
    if conditionPair[0] in conditionsDic: 
        conditionsDic[conditionPair[0]].append(conditionPair[1])
    else: 
        conditionsDic[conditionPair[0]] = [conditionPair[1]]

dictPrint(conditionsDic)
print()
print(lists)
print()

ans = 0
for lst in lists:
    lstDic = {}
    for i, elem in enumerate(lst):
        lstDic[elem] = i
    ans += process(conditionsDic, lstDic, lst)
print(f"ans = {ans}")

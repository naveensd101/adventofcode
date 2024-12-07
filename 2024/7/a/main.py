def printLines(lines):
    for line in lines:
        print(line)

def operationsList(binString):
    res = []
    for ch in binString:
        if ch == '0': res.append('*')
        if ch == '1': res.append('+')
    return res

def doMath(lst, opList):
    if len(lst)-1 != len(opList):
        print("You fucked up inside doMath lenth check")
        return

    ans = lst[0]
    for i, _ in enumerate(opList):
        if opList[i] == '*': ans *= lst[i+1]
        if opList[i] == '+': ans += lst[i+1]
    return ans


def allSearch(ans, lst):
    numOfBits = len(lst) - 1
    for state in range(2 ** numOfBits):
        binNumber = format(state, "0" + str(numOfBits) + "b")
        opList = operationsList(binNumber)
        if doMath(lst, opList) == ans: return True
    return False

N = 850
lines = [input() for _ in range(N)]
lines = [line.split(':') for line in lines]
lines = [[line[0], line[1].split()] for line in lines]
lines = [[int(line[0]), list(map(int, line[1]))] for line in lines]
# printLines(lines)
ans = 0
for line in lines:
    if allSearch(line[0], line[1]):
        ans += line[0]
        print(line)

print(ans)

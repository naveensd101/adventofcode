def f(str, n, arr):
    if len(str) == n-1:
        arr.append(str+"+")
        arr.append(str+"*")
        arr.append(str+"|")
        return
    
    f(str+"+",n, arr)
    f(str+"*",n, arr)
    f(str+"|",n, arr)




def fun(result, numberLine):
    spaces = len(numberLine) - 1
    arr =[]
    f("", spaces, arr)
    for i in range(len(arr)):
        ans = performOp(numberLine, arr[i])
        # print(ans)
        # print(arr)
        if ans == result:
            return True
    return False


def performOp(numberLine, opArr):
    ans = numberLine[0]
    for i in range(1, len(numberLine)):
        op = opArr[i-1]
        if op == '+':
            ans = ans + numberLine[i]
        elif op == '*':
            ans = ans * numberLine[i]
        elif op == '|':
            ans = int(str(ans) + str(numberLine[i]))
    return ans


arr =[]

lines = [input() for _ in range(850)]
lines = [line.split(':') for line in lines]
lines = [[int(line[0]), list(map(int, line[1].split()))] for line in lines]
result = [line[0] for line in lines]
numberLines = [line[1] for line in lines]

print(lines)
print(result)
print(numberLines)
finalAns=0

for i in range(len(numberLines)):
    print(i)
    if fun(result[i], numberLines[i]):
        finalAns+= result[i]

print("finalAns")
print(finalAns)



# 190: 10 19
# 3267: 81 40 27
# 83: 17 5
# 156: 15 6
# 7290: 6 8 6 15
# 161011: 16 10 13
# 192: 17 8 14
# 21037: 9 7 18 13
# 292: 11 6 16 20
import re

def main():
    rex = re.compile('mul\([0-9]{1,3},[0-9]{1,3}\)')
    ans = 0
    for _ in range(6):
        line = input()
        #print(rex.findall(line))
        for elem in rex.findall(line):
            comma = elem[4:][:-1].split(',')
            ans += int(comma[0]) * int(comma[1])
    print(ans)
main()

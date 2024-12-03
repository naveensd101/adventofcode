import re

def main():
    re_list = [
            "mul\([0-9]{1,3},[0-9]{1,3}\)",
            "do\(\)",
            "don\'t\(\)"
            ]
    rex = re.compile('|'.join(re_list))
    ans = 0
    off_flag = False
    for _ in range(6):
        line = input()
        print(rex.findall(line))
        for elem in rex.findall(line):
            if elem == "don't()":
                off_flag = True
            elif elem == "do()":
                off_flag = False
            else:
                if not(off_flag):
                    comma = elem[4:][:-1].split(",")
                    ans += int(comma[0]) * int(comma[1])
    print(ans)
main()

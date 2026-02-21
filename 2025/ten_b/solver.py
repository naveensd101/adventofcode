import pulp as p

n = int(input())
answers = []
for _ in range(n):
    line = input()
    lst = line.split(" ")
    lst = lst[1:]
    buttons = lst[:-1]
    buttons = [x[1:-1] for x in buttons]
    buttons = [x.split(',') for x in buttons]
    buttons = [ [int(x) for x in y] for y in buttons]
    req = lst[-1]
    req = req[1:-1]
    req = req.split(",")
    req = [int(x) for x in req]
    # print(buttons)
    # print(req)

    iplst = []
    for i in range(len(req)):
        tmp = []
        tmp.append(req[i])
        tmp.append([x for x in range(len(buttons)) if buttons[x].count(i) > 0])
        iplst.append(tmp)

    Lp_prob = p.LpProblem('Problem', p.LpMinimize)
    x = p.LpVariable.dicts("x", range(len(buttons)), lowBound=0, cat="Integer")

    #objective funciton
    Lp_prob += p.lpSum([x[i] for i in range(len(buttons))])

    #constraints
    for line in iplst:
        Lp_prob += p.lpSum([x[i] for i in line[1]]) <= line[0]
        Lp_prob += p.lpSum([x[i] for i in line[1]]) >= line[0]
    solver = p.PULP_CBC_CMD()
    status = Lp_prob.solve(solver)
    answers.append(p.value(Lp_prob.objective))
    xs = [p.value(x[i]) for i in range(len(buttons))]
    # print(xs)
    # print(iplst)
    # print(buttons)
    # print(req)
print(answers)
total = 0
for val in answers:
    total += val
print("sum = ", total)

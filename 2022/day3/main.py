lines = [line.strip() for line in open("in.1")]
score = 0
for line in lines:
    mid = len(line) // 2
    itc = set(line[:mid]).intersection(set(line[mid:]))
    for n in itc:
        D = ord(n) - 64
        prio = 0
        if n.islower():
            prio = D - 25 - 7
        else:
            prio = D + 26
        score += prio

print(score)

counter = 0
tmp = []
group = []
for line in lines:
    counter += 1
    tmp.append(line)
    if counter == 3:
        group.append(tmp)
        counter = 0
        tmp = []

score = 0
for g in group:
    itc = set(g[0]).intersection(set(g[1]).intersection(g[2]))
    c = itc.pop()
    D = ord(c) - 64
    prio = 0
    if c.islower():
        prio = D - 25 - 7
    else:
        prio = D + 26
    score += prio

print(score)

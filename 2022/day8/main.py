from pprint import pprint
from collections import defaultdict

lines = [ line.strip() for line in open("in") ]
R = len(lines)
C = len(lines[0])
G = [[c for c in row] for row in lines]

ans = 0
DIR =  [(1, 0), (-1, 0), (0, 1), (0, -1)]
p2_ans = defaultdict()

for r in range(R):
    for c in range(C):
        vis = False
        p2_ans[(r, c)] = 1
        for (dr, dc) in DIR:
            rr = r
            cc = c
            ok = True
            amount = 0
            while True:
                rr += dr
                cc += dc
                if rr < 0 or rr >= R or cc < 0 or cc >= C:
                    break
                amount += 1
                if G[rr][cc] >= G[r][c]:
                    ok = False
                    break

            p2_ans[(r, c)] *= amount

            if ok:
                vis = True

        if vis:
            ans+=1
print(ans)
print(max(p2_ans.values()))

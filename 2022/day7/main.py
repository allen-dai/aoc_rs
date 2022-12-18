from pprint import pprint
from collections import defaultdict

lines = [ line.strip().split(" ") for line in open("in") ]

path = []

size = defaultdict(int)
for line in lines:
    if line[0] == "$":
        if line[1] == "cd":
            if line[2] == "..":
                path.pop()
                continue
            path.append(line[2])
        else:
            continue
    else:
        if line[0] == "dir": continue
        for i in range(len(path)+1):
            size["/".join(path[:i])] += int(line[0])

ans = 0
for k, v in size.items():
    if v <= 100000:
        ans += v

print(ans)

LIMIT = 70000000
SIZE = LIMIT - size["/"]
NEEDED = 30000000 - SIZE
srt = sorted(size.items(), key=lambda item:item[1])

ans = 0
for (k, v) in srt:
    if v >= NEEDED:
        ans = v
        break

print(ans)

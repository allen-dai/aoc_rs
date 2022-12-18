import sys
from pprint import pprint

f = sys.argv[1] if len(sys.argv) > 1  else "in"
lines = [line.strip() for line in open(f)]

X = 1
interval = [40, 80, 120, 160, 200, 240]
ret = 0
cycle = 1
R = ""
print(R)
def idx(n):
    return (n % 40 if n % 40 != 0 else 40)

for (i, line) in enumerate(lines):
    if line == "noop":
        cycle += 1
        if 1 < idx(cycle) > X and abs(idx(cycle) - X) <= 3:
            R += "#"
        else:
            R += " "
        if cycle-1 in interval:
            print(R)
            R = ""
        continue
    for _ in range(2):
        cycle+= 1
        if 1 < idx(cycle) > X and abs(idx(cycle) - X) <= 3:
            R += "#"
        else:
            R += " "
        if cycle-1 in interval:
            print(R)
            R = ""

    (addr, val) = line.split()
    X += int(val)

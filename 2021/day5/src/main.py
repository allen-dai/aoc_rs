import sys

infile = sys.argv[1] if len(sys.argv) > 1 else 'input.txt'


G = {}

for line in open(infile):
    start, end = line.split("->")
    x1,y1 = start.split(",")
    x2,y2 = end.split(",")
    x1 = int(x1)
    x2 = int(x2)
    y1 = int(y1)
    y2 = int(y2)

    for x in range(x1, x2+1):
        for y in range(y1, y2+1):
            if (x,y) not in G:
                G[(x,y)] = 0
            G[(x,y)] += 1


print(G)

from collections import deque
from pprint import pprint

lines = [line.strip() for line in open("in")]
Q = deque()
curr = (0, 0)
moves = deque([(0, 0)])
ans = 0
for line in lines:
    mov, dis = line.split(" ")
    dis = int(dis)
    for i in range(dis):
        if mov == "U":
            curr = (curr[0] + 1, curr[1])
        if mov == "D":
            curr = (curr[0] - 1, curr[1])
        if mov == "L":
            curr = (curr[0], curr[1] - 1)
        if mov == "R":
            curr = (curr[0], curr[1] + 1)

        moves.append(curr)

for i in range(9):
    T = (0,0)
    P = (0,0)
    newMoves = deque([(0, 0)])
    while moves:
        H = moves.popleft()
        dr = abs(H[0] - T[0])
        dc = abs(H[1] - T[1])
        if dr >= 2 and dc >= 2:
            T = (H[0] - 1 if T[0] < H[0] else H[0] + 1, H[1] - 1 if T[1]<H[1] else H[1]+1)
        elif dr >= 2:
            T = (H[0] - 1 if T[0] < H[0] else H[0] + 1, H[1] )
        elif dc >= 2:
            T = (H[0], H[1] - 1 if T[1] < H[1] else H[1] +1)
        newMoves.append(T)
    moves = newMoves
    print(len(set(newMoves)))

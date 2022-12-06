from pprint import pprint
from collections import deque
from copy import deepcopy

lines = [ line for line in open("in") ]
mid = lines.index("\n")
crates_raw = lines[:mid]
moves_raw = lines[mid:]
stack_nums = [ n for n in crates_raw[-1] if n.isdigit() ]

stacks = deque()

max_stack_height = len(crates_raw) - 1 # last one is the numbers
for c in stack_nums:
    row = deque()
    for r in range(max_stack_height):
        idx = crates_raw[-1].index(c)
        if idx > len(crates_raw[r]):
            continue
        if crates_raw[r][idx].isalpha():
            row.append(crates_raw[r][idx])
    row.reverse()
    stacks.append(row)

moves = [ line.strip().split(" ") for line in moves_raw if len(line)>1]
moves = [ [int(line[1]), int(line[3]), int(line[5])] for line in moves]

stacks_p2 = deepcopy(stacks)

for num_move, frm, to in moves:
    for _ in range(num_move):
        stacks[to-1].append(stacks[frm-1].pop())

print("".join([ s[-1] for s in stacks if len(s) > 0]))


for num_move, frm, to in moves:
    tmp = []
    for _ in range(num_move):
        tmp.append(stacks_p2[frm-1].pop())
    tmp.reverse()
    stacks_p2[to-1].extend(tmp)

print("".join([ s[-1] for s in stacks_p2 if len(s) > 0]))

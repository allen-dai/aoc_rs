from pprint import pprint
from collections import deque

lines = [line for line in open("in")]
line = lines[0]

def subroutine(marker_length):
    start_signal = deque()
    for i, c in enumerate(line):
        if len(start_signal) == marker_length: start_signal.popleft()
        start_signal.append(c)

        if len(start_signal) == marker_length and len(set(start_signal)) == marker_length:
            pprint(start_signal)
            print(i+1)
            return
# part 1
subroutine(4)
# part 2
subroutine(14)

from pprint import pprint

lines = [line.strip() for line in open("in")]
pairs = []
ret = 0

for line in lines:
    left, right = line.split(",")
    start1, end1 = left.split("-")
    start2, end2 = right.split("-")
    start1 = int(start1)
    start2 = int(start2)
    end1 = int(end1)
    end2 = int(end2)

    pairs.append([(start1, end1), (start2, end2)])

for (start1, end1), (start2, end2) in pairs:
    if (start1 <= start2 and end1 >= end2) or (start2 <= start1 and end2 >= end1):
        ret += 1
print(ret)

ret = 0

for (start1, end1), (start2, end2) in pairs:
    left = set(range(start1, end1+1))
    right = set(range(start2, end2+1))
    if any(left.intersection(right)):
        ret += 1

print(ret)



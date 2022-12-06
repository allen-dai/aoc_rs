f = open("./in.1")
L = {"A": 1, "B": 2, "C": 3 }
R = {"X": 1, "Y": 2, "Z": 3 }
WIN = set(["YA", "ZB", "XC"])
content = [line.strip() for line in f]
score = 0

for line in content:
    l, r = line.split(" ")
    if r+l in WIN:
        score += R[r] + 6
    elif R[r] == L[l]:
        score += R[r] + 3
    else:
        score += R[r]

print(score)

S  = {
        "AX": 3, "AY": 4, "AZ": 8,
        "BX": 1, "BY": 5, "BZ": 9,
        "CX": 2, "CY": 6, "CZ": 7,
        }

score = 0
for line in content:
    l, r = line.split(" ")
    score += S[l+r]

print(score)

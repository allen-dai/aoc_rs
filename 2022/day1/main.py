file = open("./in.1", "r")
elf = []
tmp = []
lastline = ""
for line in file:
    line = line.strip()
    lastline = line
    if line == '':
        elf.append(tmp)
        tmp = []
        continue
    tmp.append(int(line))
elf.append([int(lastline)])

result = [sum(food) for food in elf ]
result.sort()
result.reverse()

print(f"part 1: {max(result)}")
print(f"part 2: {sum(result[0:3])}")

import re
sum = 0
for m in re.findall("mul\(([0-9]+)\,([0-9]+)\)", open("input.txt").read()):
    sum += int(m[0]) * int(m[1])
print(sum)

enabled = True
sum = 0
for m in re.findall("mul\(([0-9]+)\,([0-9]+)\)|(do)\(\)|(don)'t\(\)", open("input.txt").read()):
    if m[2] == 'do':
        enabled = True
    elif m[3] == 'don':
        enabled = False
    elif enabled:
        sum += int(m[0]) * int(m[1])
print(sum)

inp = [[int(i) for i in l.split()] for l in open("input.txt").readlines()]
difs = [[r - l for (l, r) in zip(pat[0:-1], pat[1:])] for pat in inp]

def check_script(d):
    all_inreasing = all([t > 0 and t <=3 for t in d])
    all_decreasing = all([t < 0 and t >= -3 for t in d])

    return all_inreasing + all_decreasing

s = 0
for d in difs:
    s += check_script(d)

print(s)

from itertools import combinations

s = 0
for (i, d) in zip(inp, difs):
    print(d)
    if check_script(d):
        s += 1
        continue

    for c in combinations(i, len(i) - 1):
        c_difs = [r - l for (l, r) in zip(c[0:-1], c[1:])]

        if check_script(c_difs):
            s += 1
            break

print(s)
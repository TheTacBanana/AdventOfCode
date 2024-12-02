f = open("input.txt").readlines()

lf = []
rf = []

for l in f:
    l, r = l.split();
    lf.append(int(l))
    rf.append(int(r))

lf.sort()
rf.sort()

sum = 0
for (l, r) in zip(lf, rf):
    sum += abs(r - l)

print(sum)

d = {}
for i in rf:
    if i not in d:
        d[i] = 1
    else:
        d[i] += 1

sum = 0
for l in lf:
    sum += l * d.get(l, 0)

print(sum)

deps, updates = open("input.txt").read().split("\n\n")
deps = [(int((d := dep.split("|"))[0]), int(d[1])) for dep in deps.splitlines()]
updates = [[int(i) for i in update.split(",")] for update in updates.splitlines()]

before = {}
after = {}
for (l, r) in deps:
    if r in before:
        before[r].add(l)
    else:
        before[r] = {l}

    if l in after:
        after[l].add(r)
    else:
        after[l] = {r}

failed = []
sum = 0
for update in updates:
    for (i, u) in enumerate(update):
        if u in after:
            found_dep = False
            for f in update[:i]:
                if f in after[u]:
                    found_dep = True
                    break
            if found_dep:
                failed.append(update)
                break
    else:
        sum += update[len(update)//2]

print(sum)

sum = 0
for f in failed:
    def dfs(graph, source, stack, visited):
        visited.add(source)
        for neighbour in graph[source]:
            if neighbour not in visited:
                dfs(graph, neighbour, stack, visited)

        stack.append(source)

    def topo_sort(graph):
        stack = []
        visited = set()
        for vertex in graph.keys():
            if vertex not in visited:
                dfs(graph, vertex, stack, visited)

        return stack

    out = topo_sort({i: {s for s in before.get(i, set()) if s in set(f)} for i in f})
    sum += out[len(out) // 2]
print(sum)
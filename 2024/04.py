from copy import deepcopy

grid = [[j for j in i.strip()] for i in open("input.txt").readlines()]
t_grid = list(map(list, zip(*grid)))

def diagonals(g):
    def get_diag(r, c):
        diagonal = []
        while r < len(g) and c < len(g[0]):
            diagonal.append(g[r][c])
            r += 1
            c += 1
        return diagonal

    for row in range(len(g)):
        yield get_diag(row, 0)
    for col in range(1, len(g[0])):
        yield get_diag(0, col)

fdag = [*diagonals(grid)]
bdag = [*diagonals([r[::-1] for r in grid])]

def test(g, word):
    split_word = [i for i in word]
    rev_word = deepcopy(split_word)
    rev_word.reverse()

    count = 0
    for row in g:
        for i in range(len(row) - len(word) + 1):
            if row[i:(i+len(word))] == split_word or row[i:(i+len(word))] == rev_word:
                count += 1
    return count

sum = test(grid, "XMAS") + test(t_grid, "XMAS") + test(fdag, "XMAS") + test(bdag, "XMAS")
print(sum)

variants = ["M.S.A.M.S", "S.M.A.S.M", "M.M.A.S.S", "S.S.A.M.M"]
sum = 0
for i in range(len(grid) - 2):
    for j in range(len(grid[0]) - 2):
        section = [x for xs in [row[j:j+3] for row in grid[i:i+3]] for x in xs]
        for v in variants:
            for (l, r) in zip(v, section):
                if l == ".":
                    continue
                elif l != r:
                    break
            else:
                sum += 1
print(sum)
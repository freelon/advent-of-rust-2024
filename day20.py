def parse_input(input: list[str]):
    l = [(x, y) for (y, line) in enumerate(input) for (x, c) in enumerate(line) if c == '#']
    start = [(x, y) for (y, line) in enumerate(input) for (x, c) in enumerate(line) if c == 'S'][0]
    goal = [(x, y) for (y, line) in enumerate(input) for (x, c) in enumerate(line) if c == 'E'][0]
    w = max([x for (x, y) in l])
    h = max([y for (x, y) in l])
    return l, w, h, start, goal


def sssp(points, w, h, start):
    visited = dict()
    (sx, sy) = start
    stack = [(sx, sy, 0)]
    while len(stack) > 0:
        (x, y, c) = stack.pop(0)

        if (x, y) in visited:
            continue
        visited[(x, y)] = c

        for (a, b) in [(x+nx, y+ny) for (nx, ny) in [(0, 1), (1, 0), (0, -1), (-1, 0)]]:
            if 0 <= a <= w and 0 <= b <= w and (a, b) not in points:
                stack.append((a, b, c + 1))

    return visited


def mhd(a, b):
    (xa, ya) = a
    (xb, yb) = b
    return abs(xa - xb) + abs(ya - yb)


def others(origin, max_distance):
    (sx, sy) = origin
    return [(a, b) for a in range(sx - max_distance, sx + max_distance + 1) for b in range(sy - max_distance, sy + max_distance + 1)]


def solve(points, threshold, cheat_length):
    fromstart = sssp(set(points), w, h, start)
    toend = sssp(set(points), w, h, goal)
    cheatrange = range(2, cheat_length + 1)
    return len([1 for a in fromstart for b in others(a, cheat_length) if mhd(a, b) in cheatrange and b in toend and fromstart[a] + mhd(a, b) + toend[b] <= threshold])


source = "input/day20.txt"
with open(source, 'r') as f:
    data = f.read()

(points, w, h, start, goal) = parse_input(data.strip().split('\n'))
bottomline = sssp(set(points), w, h, start)[goal]

solution1 = solve(set(points), bottomline - 100, 2)
print("part 1: " + str(solution1))

solution2 = solve(set(points), bottomline - 100, 20)
print("part 2: " + str(solution2))

def parse_input(input: list[str]):
    l = [(x, y) for (y, line) in enumerate(input) for (x, c) in enumerate(line) if c == '#']
    start = [(x, y) for (y, line) in enumerate(input) for (x, c) in enumerate(line) if c == 'S'][0]
    goal = [(x, y) for (y, line) in enumerate(input) for (x, c) in enumerate(line) if c == 'E'][0]
    w = max([x for (x, y) in l])
    h = max([y for (x, y) in l])
    return l, w, h, start, goal


def sp(points, w, h, start, goal):
    visited = dict()
    (sx, sy) = start
    stack = [(sx, sy, 0)]
    while len(stack) > 0:
        (x, y, c) = stack.pop(0)
        if (x, y) == goal:
            return c

        if (x, y) in visited:
            continue
        visited[(x, y)] = c

        for (a, b) in [(x+nx, y+ny) for (nx, ny) in [(0, 1), (1, 0), (0, -1), (-1, 0)]]:
            if 0 <= a <= w and 0 <= b <= w and (a, b) not in points:
                stack.append((a, b, c + 1))

    return None


source = "input/day20.txt"
with open(source, 'r') as f:
    data = f.read()

(points, w, h, start, goal) = parse_input(data.strip().split('\n'))
bottomline = sp(set(points), w, h, start, goal)
print(bottomline)
atleast100 = 0
for p in points:
    remains = [x for x in points if p != x]
    pathlength = sp(set(remains), w, h, start, goal)
    if bottomline - pathlength >= 100:
        atleast100 += 1


print("task 1: " + str(atleast100))

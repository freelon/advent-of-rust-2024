def parse_input(input: list[str]):
    return [tuple(map(lambda f: int(f), cl.split(','))) for cl in input]


with open('input/day18.txt', 'r') as f:
    data = f.read()


points = parse_input(data.strip().split('\n'))
points = set(points[:1024])
start = (0, 0)
goal = (70, 70)
w = 70

# points = set([
#     (5, 4),
#     (4, 2),
#     (4, 5),
#     (3, 0),
#     (2, 1),
#     (6, 3),
#     (2, 4),
#     (1, 5),
#     (0, 6),
#     (3, 3),
#     (2, 6),
#     (5, 1),
#     (1, 2),
#     (5, 5),
#     (2, 5),
#     (6, 5),
#     (1, 4),
#     (0, 4),
#     (6, 4),
#     (1, 1),
#     (6, 1),
#     (1, 0),
#     (0, 5),
#     (1, 6),
#     (2, 0),
# ][:12])
# goal = (6, 6)
# w = 6

visited = dict()
stack = [(0, 0, 0)]
while len(stack) > 0:
    # (x, y, c) = stack.pop(len(stack)-1)
    (x, y, c) = stack.pop(0)
    print("took " + str((x, y, c)))
    if (x, y) == goal:
        print(c)
        break

    if (x, y) in visited:
        continue
    visited[(x, y)] = c

    for (a, b) in [(x+nx, y+ny) for (nx, ny) in [(0, 1), (1, 0), (0, -1), (-1, 0)]]:
        print("checking " + str((a, b)))
        if 0 <= a <= w and 0 <= b <= w and (a, b) not in points:
            stack.append((a, b, c + 1))
            print("added something")

# 2224 too high

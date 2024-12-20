def parse_input(input: list[str]):
    return [tuple(map(lambda f: int(f), cl.split(','))) for cl in input]


with open('input/day18.txt', 'r') as f:
    data = f.read()


points = parse_input(data.strip().split('\n'))

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


def sp(points, w):
    visited = dict()
    stack = [(0, 0, 0)]
    while len(stack) > 0:
        (x, y, c) = stack.pop(0)
        if (x, y) == (w, w):
            return c

        if (x, y) in visited:
            continue
        visited[(x, y)] = c

        for (a, b) in [(x+nx, y+ny) for (nx, ny) in [(0, 1), (1, 0), (0, -1), (-1, 0)]]:
            if 0 <= a <= w and 0 <= b <= w and (a, b) not in points:
                stack.append((a, b, c + 1))

    return None


print("task 1: " + str(sp(set(points[:1024]), 70)))

for take in range(1024, len(points)):
    solution = sp(set(points[:take]), 70)
    if not solution:
        print("task 2: " + str(points[take - 1]))
        break

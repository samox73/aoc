from aocd.models import Puzzle
from aocd import submit

data = Puzzle(year=2022, day=2).input_data
print(data)

hand_points = {"X": 1, "Y": 2, "Z": 3}
left_to_right_map = {"A": "X", "B": "Y", "C": "Z"}


def get_round_points(left, right):
    if left_to_right_map[left] == right:
        return 3
    elif (
        left == "A"
        and right == "Y"
        or left == "B"
        and right == "Z"
        or left == "C"
        and right == "X"
    ):
        return 6
    else:
        return 0


solution_a = 0
for line in data.split("\n"):
    left, right = line.split(" ")
    pts = get_round_points(left, right)
    pts += hand_points[right]
    print(f"{left} vs {right}: {pts}")
    solution_a += pts

submit(solution_a, part="a", year=2022, day=2)

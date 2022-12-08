from aocd.models import Puzzle
from aocd import submit

data = Puzzle(year=2022, day=2).input_data
print(data)

hand_map = {
    "A": {0: "C", 3: "A", 6: "B"},
    "B": {0: "A", 3: "B", 6: "C"},
    "C": {0: "B", 3: "C", 6: "A"},
}
hand_points = {"A": 1, "B": 2, "C": 3}
outcome_to_points_map = {"X": 0, "Y": 3, "Z": 6}


def determine_hand(left, needed_points):
    return hand_map[left][needed_points]


def get_round_points(left, right):
    if left == right:
        return 3
    elif (
        left == "A"
        and right == "B"
        or left == "B"
        and right == "C"
        or left == "C"
        and right == "A"
    ):
        return 6
    else:
        return 0


solution = 0
for line in data.split("\n"):
    left, right = line.split(" ")
    hand = determine_hand(left, outcome_to_points_map[right])
    pts = get_round_points(left, hand)
    pts += hand_points[hand]
    solution += pts

print(solution)
submit(solution, part="b", year=2022, day=2)

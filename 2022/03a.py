from aocd.models import Puzzle
from aocd import submit

data = Puzzle(year=2022, day=3).input_data


def getValueOfItem(c):
    if c.islower():
        return ord(c) - ord("a") + 1
    else:
        return ord(c) - ord("A") + 27


# Test input with result 157
# data = """vJrwpWtwJgWrhcsFMMfFFhFp
# jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
# PmmdzqPrVvPwwTWBwg
# wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
# ttgJtRGJQctTZtZT
# CrZsJsPPZsGzwwsLwLmpwMDw"""

solution = 0
for line in data.split("\n"):
    left = line[: len(line) // 2]
    right = line[len(line) // 2 :]
    print(f"{left} | {right}")
    item = ""
    for char in left:
        if char in right:
            item = char
    print(f"{item} -> {getValueOfItem(item)}")
    solution += getValueOfItem(item)

print(solution)
submit(solution, part="a", year=2022, day=3)

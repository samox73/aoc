from aocd.models import Puzzle
from aocd import submit

data = Puzzle(year=2022, day=3).input_data


def getValueOfItem(c):
    if c.islower():
        return ord(c) - ord("a") + 1
    else:
        return ord(c) - ord("A") + 27


def intersection(l1, l2):
    result = []
    for char in l1:
        if char in l2:
            result.append(char)
    return result

# data = """vJrwpWtwJgWrhcsFMMfFFhFp
# jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
# PmmdzqPrVvPwwTWBwg
# wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
# ttgJtRGJQctTZtZT
# CrZsJsPPZsGzwwsLwLmpwMDw"""

solution = 0
lines = data.split("\n")
for n in range(0, len(lines), 3):
    line1, line2, line3 = lines[n : n + 3]
    intersection1 = intersection(line1, line2)
    intersection2 = intersection(intersection1, line3)
    key = intersection2[0]
    print(line1)
    print(line2)
    print(line3)
    print(key)
    print()
    solution += getValueOfItem(key)

print(solution)
submit(solution, part="b", year=2022, day=3)

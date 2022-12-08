from aocd.models import Puzzle
from aocd import submit

data = Puzzle(year=2022, day=1).input_data

# ============== PART A ==============
groups = []
current_group = 0
for line in data.split("\n"):
    if len(line) > 0:
        current_group += float(line)
    else:
        groups.append(current_group)
        current_group = 0

solution_a = f"{max(groups):.0f}"
print(solution_a)
# submit(solution_a, part="a", year=2022, day=1)

# ============== PART B ==============
groups.sort(reverse=True)
solution_b = sum(groups[0:3])
solution_b = f"{solution_b:.0f}"
print(solution_b)
submit(solution_b, part="b", year=2022, day=1)
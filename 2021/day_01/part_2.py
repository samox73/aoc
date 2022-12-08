import numpy as np

array = np.loadtxt("input.txt", dtype=np.int32)

count = 0
for index in range(1, len(array)):
    sum_A = sum(array[index - 1 : index + 2])
    sum_B = sum(array[index : index + 3])
    if sum_B > sum_A:
        count += 1
        
print(count)
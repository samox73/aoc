import numpy as np

array = np.loadtxt("input.txt", dtype=np.int32)

count = 0
for index in range(len(array)):
    if array[index] > array[index - 1]:
        count += 1
        
print(count)
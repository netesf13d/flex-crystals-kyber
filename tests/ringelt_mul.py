# -*- coding: utf-8 -*-
"""
Created on Thu Sep 12 15:43:35 2024

@author: netes
"""

p = 7681

p1 = [0, 2000, 7680, 0]
p2 = [1, 1000, 2, 1]

# p1 = [1, 0, 0, 0]
# p2 = [1, 1000, 2, 1]

temp = [0]*7

for k in range(7):
    for i in range(4):
        for j in range(4):
            if i+j == k:
                temp[k] += (p1[i] * p2[j]) % p

print(temp)
prod = temp[:4]
for i in range(3):
    prod[i] -= temp[4+i]

prod = [x%p for x in prod]
print(prod)
                
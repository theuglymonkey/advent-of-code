import sys
import os
from operator import methodcaller
from itertools import groupby

directory = os.getcwd()
pred = lambda x: x == " "
with open('elfs.txt') as fp:
    lines = fp.read().split('\n')
    chunks = (list(g) for k,g in groupby(lines, key=lambda x: x != '') if k)
    reducedList = []
    for chunk in chunks:
        to_ints = [eval(i) for i in chunk]
        reduced = sum(to_ints)
        reducedList.append(reduced)
    maxV = max(reducedList)
    reducedList.sort(reverse=True)
    print(reducedList[0] + reducedList[1] + reducedList[2])
    
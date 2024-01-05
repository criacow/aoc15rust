#!/usr/bin/python3
from json import loads
import sys

def process(j):
    if type(j) == int:
        return j
    if type(j) == list:
        sum = 0
        for k in j:
            sum += process(k)
        return sum
    if type(j) == dict:
        if 'red' in j.values():
            return 0
        sum = 0
        for k in j.values():
            sum += process(k)
        return sum
    return 0

for line in sys.stdin:
    jsn = loads(line)

k = process(jsn)
print(k)

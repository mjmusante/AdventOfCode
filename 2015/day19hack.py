#! /usr/bin/python

import re

with open('day19.txt') as f:
    input = f.read()


xinput = """e => H
e => O
H => HO
H => OH
O => HH

HOH"""

molecule = input.split('\n')[-1][::-1]
reps = {m[1][::-1]: m[0][::-1] 
        for m in re.findall(r'(\w+) => (\w+)', input)}
def rep(x):
    return reps[x.group()]

count = 0
while molecule != 'e':
    molecule = re.sub('|'.join(reps.keys()), rep, molecule, 1)
    count += 1
    print(count)

print(count)

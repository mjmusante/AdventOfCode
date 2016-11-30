#! /usr/bin/python

with open("day23.txt") as f:
    lines = f.read().splitlines()

mem = []

for l in lines:
    x = l.split(" ")
    if x[1].startswith("a"):
        x[1] = "a"
    elif x[1].startswith("b"):
        x[1] = "b"
    mem.append(x)

reg = { "a": 1, "b": 0 }
inst = 0
while inst >= 0 and inst < len(mem):
    # print("%s> %s/%s | %s" % (inst, reg["a"], reg["b"], mem[inst]))
    nexti = inst + 1
    if mem[inst][0] == "hlf":
        reg[mem[inst][1]] /= 2
    elif mem[inst][0] == "tpl":
        reg[mem[inst][1]] *= 3
    elif mem[inst][0] == "inc":
        reg[mem[inst][1]] += 1
    elif mem[inst][0] == "jmp":
        nexti = inst + int(mem[inst][1])
    elif mem[inst][0] == "jio" and reg[mem[inst][1]] == 1:
        nexti = inst + int(mem[inst][2])
    elif mem[inst][0] == "jie" and reg[mem[inst][1]] & 1 == 0:
        nexti = inst + int(mem[inst][2])
    inst = nexti

print(reg)

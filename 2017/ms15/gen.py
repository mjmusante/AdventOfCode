import sys


def judge(a, b, lim=40000000):
    count = 0
    for i in range(lim):
        a = (a * 16807) % 2147483647
        b = (b * 48271) % 2147483647
        if (a & 0xffff) == (b & 0xffff):
            count += 1
    return count

def judge_gen(a, b):
    def next_a(a):
        while True:
            a = (a * 16807) % 2147483647
            if a & 0x3 == 0:
                return a

    def next_b(b):
        while True:
            b = (b * 48271) % 2147483647
            if b & 0x7 == 0:
                return b                

    count = 0
    for i in range(5000000):
        a = next_a(a)
        b = next_b(b)
        if (a & 0xffff) == (b & 0xffff):
            count += 1
    return count


if judge(65, 8921, 5) != 1:
    print("Test 1: fail")
    sys.exit(1)
print("Test 1: pass")

if judge(65, 8921) != 588:
    print("Test 2: fail")
    sys.exit(1)
print("Test 2: pass")

if judge_gen(65, 8921) != 309:
    print("Test 3: fail")
    sys.exit(1)
print("Test 3: pass")

# puzzle input:
# Generator A starts with 703
# Generator B starts with 516
print("Part 1: %s" % judge(703, 516))
print("Part 2: %s" % judge_gen(703, 516))

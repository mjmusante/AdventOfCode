import re

ARROWD = re.compile(r'^([a-z]*) \((\d*)\) -> ([a-z, ]*)$')
CLEAN = re.compile(r'^([a-z]*) \((\d*)\)$')


class Node:

    def __init__(self, base, weight, children):
        self.base = base
        self.weight = weight
        self.weight_carried = -1
        self.children = children


class Tree:

    def __init__(self):
        self.tree = dict()

    def add_entry(self, base, weight, children=[]):
        node = Node(base, weight, children)
        self.tree[base] = node

    def sum_weights(self, start):
        weight = self.tree[start].weight
        if len(self.tree[start].children) > 0:
            for c in self.tree[start].children:
                weight += self.sum_weights(c)
        self.tree[start].weight_carried = weight
        return weight


def find_bottom_of(ary):
    found = dict()
    for i in ary:
        m = ARROWD.match(i)
        if m:
            left = m.group(1)
            right = m.group(3).split(", ")
            if left in found:
                found[left] = 0
            else:
                found[left] = 1
            for j in right:
                found[j] = 0
        else:
            m = CLEAN.match(i)
            found[m.group(1)] = 0
    for k in found.keys():
        if found[k] == 1:
            return k
    return "*FAIL*"


def bad_weight(ary, bottom):
    t = Tree()
    for i in ary:
        m = CLEAN.match(i)
        if m:
            t.add_entry(m.group(1), int(m.group(2)))
        else:
            m = ARROWD.match(i)
            base = m.group(1)
            weight = int(m.group(2))
            children = m.group(3).split(", ")
            t.add_entry(base, weight, children)
    # t.link_children()
    t.sum_weights(bottom)

    pos = bottom
    prev_valid = 0
    while True:
        theory = dict()
        practice = dict()
        for c in t.tree[pos].children:
            w = t.tree[c].weight_carried
            if w in practice:
                # we've already eliminated this weight
                if len(theory) == 1:
                    # but we've got one other different weight - it wins
                    break
            elif w in theory:
                # we've seen this weight before - eliminate it from contention
                theory.pop(w)
                practice[w] = c
                if len(theory) == 1:
                    # if pulling this weight out of theory means there's one
                    # left, then it's automatically the winner
                    break
            else:
                theory[w] = c
                if len(practice) > 0:
                    # if we've found other entries with a weight that's
                    # different to this one, then this one wins
                    break
        if len(theory) == 0:
            # all children are balanced, so we're the odd one out; calculate
            # the value it should have been
            return t.tree[pos].weight - t.tree[pos].weight_carried + prev_valid

        # we should only have found one out of balance
        if len(theory) != 1:
            print('length of theory dict is invalid')
            return t

        # follow the inbalance up the tree
        prev_valid = practice.keys()[0]
        pos = theory.values()[0]


TESTDATA = [
    "pbga (66)",
    "xhth (57)",
    "ebii (61)",
    "havc (66)",
    "ktlj (57)",
    "fwft (72) -> ktlj, cntj, xhth",
    "qoyq (66)",
    "padx (45) -> pbga, havc, qoyq",
    "tknk (41) -> ugml, padx, fwft",
    "jptl (61)",
    "ugml (68) -> gyxo, ebii, jptl",
    "gyxo (61)",
    "cntj (57)",
]

if __name__ == "__main__":
    lines = TESTDATA
    bot = find_bottom_of(lines)
    # print("Test: bottom of tree is %s" % bot)
    assert(bot == 'tknk')
    correct_weight = bad_weight(lines, bot)
    assert(correct_weight == 60)
    # print("Test: incorrect weight should be %s" % correct_weight)

    lines = [line.strip() for line in open("puzzle_data.txt")]
    bot = find_bottom_of(lines)
    print("Part 1: %s" % bot)
    print("Part 2: %s" % bad_weight(lines, bot))

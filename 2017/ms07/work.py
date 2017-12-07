import re

ARROWD = re.compile(r'([a-z]*) \((\d*)\) -> ([a-z, ]*)$')
CLEAN = re.compile(r'([a-z]*) \((\d*)\)$')

class Node:
    def __init__(self, base, weight, children):
        self.base = base
        self.weight = weight
        self.weight_carried = -1
        self.parent = None
        self.children = children

    def add_parent(self, parent):
        self.parent = parent

class Tree:
    def __init__(self):
        self.tree = dict()

    def add_entry(self, base, weight, children=[]):
        node = Node(base, weight, children)
        self.tree[base] = node

    def link_children(self):
        for k in self.tree.keys():
            for c in self.tree[k].children:
                self.tree[c].add_parent(k)

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

def weights(ary, bottom):
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
    t.link_children()
    t.sum_weights(bottom)

    trial = 0
    pos = bottom
    prev_valid = 0
    while True:
        print("scanning %s" % pos)
        trial += 1
        if trial > 10: return t
        theory = dict()
        practice = dict()
        for c in t.tree[pos].children:
            w = t.tree[c].weight_carried
            if w in practice:
                print("already eliminated %s (%s)" % (w, c))
                if len(theory) == 1:
                    print("that makes %s the winner" % theory.values()[0])
                    break
                continue
            elif w in theory:
                theory.pop(w)
                practice[w] = c
                if len(theory) == 1:
                    print("unique weight %s found (%s)" %
                          (theory.keys()[0], theory.values()[0]))
                    break
                print("duplicate weight %s found (%s)" % (w, c))
            else:
                theory[w] = c
                if len(practice) > 0:
                    print("found unique weight %s (%s)" % (w, c))
                    break
                print("potential winner %s (%s)" % (w, c))
        if len(theory) == 0:
            # all children are balanced, so we're the odd one out
            print("node %s is unbalanced: should be %s but is %s" %
                (pos, prev_valid, t.tree[pos].weight_carried))
            print("\toriginal weight: %s" % t.tree[pos].weight)
            diff = t.tree[pos].weight_carried - prev_valid
            print("\tshould be: %s" % (t.tree[pos].weight - diff))
            return t
        if len(theory) == 1:
            prev_valid = practice.keys()[0]
            pos = theory.values()[0]
            continue

        print('length of theory dict is invalid')
        return t


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
    print("Test: %s" % bot)
    weights(lines, bot)
    lines = [line.strip() for line in open("puzzle_data.txt")]
    bot = find_bottom_of(lines)
    print("Part 1: %s" % bot)
    print("Part 2:")
    weights(lines, bot)
class Cksum:
    def __init__(self, lines):
        self.lines = lines

    def cksum(self):
        result = 0
        for l in self.lines:
            intarray = [int(x) for x in l.split()]
            result += max(intarray) - min(intarray)

        return result

    def even(self):
        result = 0
        for l in self.lines:
            intarray = [int(x) for x in l.split()]
            stop = False
            for i in range(len(intarray) - 1):
                for j in range(i + 1, len(intarray)):
                    if intarray[i] % intarray[j] == 0:
                        result += intarray[i] / intarray[j]
                        stop = True
                        break
                    elif intarray[j] % intarray[i] == 0:
                        result += intarray[j] / intarray[i]
                        stop = True
                        break
                if stop:
                    break

        return result


if __name__ == "__main__":
    puzzle = Cksum([line.strip() for line in open("puzzle_data.txt")])
    print("Part 1: %s" % puzzle.cksum())
    print("Part 2: %s" % puzzle.even())
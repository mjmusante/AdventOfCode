class Captcha:

    def __init__(self, code):
        self.code = code
        self.len = len(code)

    def sumMatch(self):
        result = 0
        for i in range(self.len):
            iprime = (i + 1) % self.len
            if self.code[i] == self.code[iprime]:
                result += int(self.code[i])
        return result

    def sumHalfway(self):
        result = 0
        for i in range(self.len / 2):
            iprime = (i + self.len / 2) % self.len
            if self.code[i] == self.code[iprime]:
                result += int(self.code[i])
        return result * 2


if __name__ == "__main__":
    captcha_code = open("puzzle_data.txt").read()
    print("Part 1: %s" % (Captcha(captcha_code).sumMatch()))
    print("part 2: %s" % (Captcha(captcha_code).sumHalfway()))

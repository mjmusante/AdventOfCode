import md5

from itertools import groupby

class OneTimePad:
    def __init__(self, salt):
        self.mval = md5.new(salt)
        self.rainbow = dict()

    @staticmethod
    def find_tuples(str, chars):
        rslt = []
        g = groupby(str)
        for k, v in g:
            lst = list(v)
            if len(lst) < chars:
                continue
            rslt += [(len(lst), lst[0])]
        return rslt

    def triple(self, start):
        while True:
            n = self.mval.copy()
            n.update("%s" % start)
            if max([len(list(v)) for k, v in groupby(n.hexdigest())]) > 2:
                return start
            start += 1

    @staticmethod
    def stretch(str):
        i = 0
        while i < 2016:
            n = md5.new(str)
            str = n.hexdigest()
            i += 1
        return str

    def get_md5(self, val, dostretch=False):
        if val in self.rainbow:
            return self.rainbow[val]
        n = self.mval.copy()
        n.update("%s" % val)
        rslt = n.hexdigest()
        if dostretch:
            rslt = OneTimePad.stretch(n.hexdigest())
        self.rainbow[val] = rslt
        return rslt

    def is_key(self, start, dostretch=False):
        biz = self.get_md5(start, dostretch)
        wchar = OneTimePad.find_tuples(biz, 3)
        if not wchar:
            return False
        nextval = start + 1
        while nextval <= start + 1000:
            srch = self.get_md5(nextval, dostretch)
            for i in OneTimePad.find_tuples(srch, 5):
                if i[1] == wchar[0][1]:
                    # print("Key %s %s has matching index %s %s" % (start, biz, nextval, n.hexdigest()))
                    return True
            nextval += 1
        return False

if __name__ == "__main__":
    otp = OneTimePad("ngcjuoqr")
    i = 0
    key_count = 0
    while key_count < 64:
        if otp.is_key(i, True):
            key_count += 1
            print("key %s found at index %s" % (key_count, i))
        i += 1

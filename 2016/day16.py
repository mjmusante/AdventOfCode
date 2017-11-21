import re

class DCurve:
    def __init__(self, bits):
        self.bits = bits

    def result(self):
        newbits = self.bits[::-1].replace('0', 'x') \
                                 .replace('1', '0') \
                                 .replace('x', '1')
        return self.bits + "0" + newbits

    @staticmethod
    def calc_cksum(bitstring):
        if len(bitstring) % 2 == 1:
            return False
        check = bitstring
        while len(check) % 2 == 0:
            rslt = ""
            for pair in re.findall("(?s)..", check):
                if pair == "11" or pair == "00":
                    rslt += "1"
                else:
                    rslt += "0"
            check = rslt
        return check


    def checksum(self, length):
        curlen = len(self.bits)
        while curlen < length:
            self.bits = self.result()
            curlen = len(self.bits)

        return DCurve.calc_cksum(self.bits[:length])

if __name__ == "__main__":
    dc = DCurve("10010000000110000")
    print("Checksum with length 272 is %s" % dc.checksum(272))
    dc = DCurve("10010000000110000")
    print("Checksum with length 35651584 is %s" % dc.checksum(35651584))
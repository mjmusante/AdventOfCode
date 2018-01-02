class Rogue:

    def __init__(self, row=None):
        self.row = row
        if row is None:
            self.safe_count = 0
        else:
            self.safe_count = row.count(".")

    def next_row(self):
        rslt = ""
        for i in range(len(self.row)):
            rslt += self.next_char_at(i)
        return rslt

    def next_char_at(self, idx):
        trap_info = 0       # 0bXYZ, x=left, y=middle, z=right

        if idx > 0 and self.row[idx - 1] == '^':
            trap_info |= 1

        if self.row[idx] == '^':
            trap_info |= 2

        if idx < len(self.row) - 1 and self.row[idx + 1] == '^':
            trap_info |= 4

        return ".^.^^.^."[trap_info]

    def cycle(self, count):
        for i in range(count):
            self.row = self.next_row()
            self.safe_count += self.row.count(".")

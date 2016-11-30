#! /usr/bin/python


# To continue, please consult the code grid in the manual.
# Enter the code at row 2947, column 3029.

# /   1  2  3  4  5
# 1:  1  3  6 10 15
# 2:  2  5  9 14 20
# 3:  4  8 13 19 26
# 4:  7 12 18 25 33
# 5: 11 17 24 32 41
# 6: 16 23 31 40 50

# row: start by adding one more than the row number, then inc by 1
# col: start by adding the col number, then inc by 1

# first number in row 2947 is:
# sum(range(1, 2947)) + 1 = 4340932
# moving to column 3029:

# 2947: 4340932 4343880 4346829 ... 17850354

num = 20151125
i = 0
while i < 17850353:
    num = (num * 252533) % 33554393
    i += 1
print("%s: %s" % (i, num))

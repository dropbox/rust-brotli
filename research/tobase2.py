import sys
result = []
def reverse(x):
    return x[::-1]
    ret = ''
    for y in reversed(x):
        ret += y
    return ret
for byte in sys.stdin.read():
    result.append(reverse("{0:#010b}".format(ord(byte)).replace("0b","")))
print ''.join(result)

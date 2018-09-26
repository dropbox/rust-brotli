import sys

result = []
cur_count = 0
cur_val = 0
for byte in sys.stdin.read():
    if byte == '1':
        cur_val |= (1<<cur_count)
    elif byte != '0':
        break
    cur_count += 1
    if cur_count == 8:
        result.append(chr(cur_val))
        cur_val = 0
        cur_count = 0
if cur_count != 0:
    result.append(chr(cur_val))
sys.stdout.write(''.join(result))

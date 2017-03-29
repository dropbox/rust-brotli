import re
import sys

pointer_whitelist = set([
        'use_rle_for_non_zero',
        'use_rle_for_zero',
        'self',
])

p = re.compile("""fn[^(]+[(]((([^),]*),)*)(([^),]*)[)])""")
VARIABLE_TO_DEREFERENCE="""([A-Za-z0-9_.]+|(\(\*[A-Za-z0-9_.]+\))[.][A-Za-z0-9_.]+)"""
NOT_ISIZE = """([^(]|\n|(\([^i])|(\(i[^s])|(\(is[^i]))+"""
offset_pat = VARIABLE_TO_DEREFERENCE + """[.]offset\(""" + NOT_ISIZE + """\(isize\)[ \n\t]*\)"""
o2 = """\*""" + offset_pat
p2 = re.compile(o2)
pointer_cast = re.compile("""as \(\*[^\)]+\)""")
psubarray = re.compile(offset_pat)
elite_something = """if[\n ]+1337i?3?2?[\n ]*!=[\n ]*0[\n ]*{[\n ]*(SOMETHING([\n ]+'[a-zA-Z0-9_]+)?(\n )*;)[\n ]*}"""
elite_break = re.compile(elite_something.replace("SOMETHING", "break"))
elite_continue = re.compile(elite_something.replace("SOMETHING", "continue"))
def subbreak(match):
    return match.group(1)
def subfun(match):
    all_items = match.group()
    #if all_items.count(".offset") > 1:
    #    print "ignoring " + all_items +" for multi_item"
    #    assert not all_items
    #    return all_items # not the inner-most
    each_arg = all_items.split(',')
    out_arg = []
    for index in range(len(each_arg)):
        v = each_arg[index]
        out_arg.append(v)
        is_single = False
        for item in pointer_whitelist:
            if item in v:
                is_single = True
        where = v.find('*mut')
        if where != -1:
            if is_single:
                v= v.replace('*mut', '&mut')
            else:
                v= v.replace('*mut', '&mut [').replace('[ ','[') + ']'
        where = v.find('*const')
        if where != -1:
            if is_single:
                v= v.replace('*const', '&')
            else:
                v= v.replace('*const', '& [').replace('[ ','[')+']'
        if v.endswith(')]'):
            v = v[:-2] + '])'
        out_arg[index] = v
    return ','.join(out_arg)

def recursive_offset_match(data):
    if data.find('.offset') != -1:
        data = p2.sub(dereffun, data)
        data = psubarray.sub(desubarrayfun, data)
    return data

def zerofirstoffset(data):
    where = data.find('.offset')
    if where == -1:
        assert False
    return data[0:where] + '.\xff' + data[where + 2:]

def dereffun(match):
    data = match.group()
    data = zerofirstoffset(data)
    data = data[1:] # zero out the *
    data = data.replace('.\xffffset(','[')
    data = data.replace('(isize)', '(usize)')
    data = data[:-1] + ']'
    data = recursive_offset_match(data)
    return data

def desubarrayfun(match):
    data = match.group()
    data = zerofirstoffset(data)
    assert data.count('.\xffffset') == 1
    data = data.replace('.\xffffset(','[')
    data = data.replace('(isize)', '(usize)')
    data = data[:-1] + '..]'
    data = recursive_offset_match(data)
    return data
def balance(data):
    retlist = []
    stack = []
    bad_chars = []
    rev_paren = {
        '{':'}','}':'{',
        '[':']',']':'[',
        '(':')',')':'(',}
    while True:
        matches = [data.find(c) for c in "{}()[]"]
        where = min(x if x >= 0 else len(data) for x in matches)
        if where == len(data):
            retlist.append(data)
            data = b''
            break
        ch = data[where]
        if ch in '({[':
            stack.append(ch)
            retlist.append(data[:where+1])
            data = data[where + 1:]
        else:
            if len(bad_chars) and bad_chars[-1][-1] == rev_paren[ch]:
                retlist.append(bad_chars[-1])
                bad_chars.pop()
            if ch == rev_paren[stack[-1]]:
                retlist.append(data[:where + 1])
                data = data[where+1:]
                stack.pop()
                while len(bad_chars) and bad_chars[-1][-1] ==rev_paren[stack[-1][-1]]:
                    retlist.append(bad_chars[-1])
                    bad_chars.pop()
                    stack.pop()
            else:
                bad_chars.append(data[:where + 1])
                data = data[where+1:]
                
    return ''.join(retlist)
def rem(match):
    return ""
with open(sys.argv[1]) as f:
    ret = p.sub(subfun, f.read())
    ret = p2.sub(dereffun, ret)
    ret = psubarray.sub(desubarrayfun, ret)
    ret = ret.replace('i32 as (usize)', 'usize')
    ret = ret.replace('i32 as (u32)', 'u32')
    ret = pointer_cast.sub(rem, ret)
    ret = elite_break.sub(subbreak, ret)
    ret = elite_continue.sub(subbreak, ret)
    ret = ret.replace("#[derive(Clone, Copy)]", "")
    ret = ret.replace("#[repr(C)]", "")
    ret = ret.replace("#[no_mangle]", "")
    ret = ret.replace("unsafe extern ", "")
    ret = ret.replace("unsafe", "")
    ret = ret.replace('self', 'xself')
    ret = balance(ret)
    sys.stdout.write(ret)

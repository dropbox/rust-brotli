import re
import sys

pointer_whitelist = set([
        'use_rle_for_non_zero',
        'use_rle_for_zero',
])

p = re.compile("""fn[^(]+[(]((([^),]*),)*)(([^),]*)[)])""")
offset_pat = """[A-Za-z0-9_]+[.]offset\(.+\(isize\)[ \n\t]*\)"""
o2 = """\*""" + offset_pat
p2 = re.compile(o2)
psubarray = re.compile(offset_pat)
def subfun(match):
    each_arg = match.group().split(',')
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


def dereffun(match):
    data = match.group()
    print "OURND MATVCH",data
    data = data[1:] # zero out the *
    data = data.replace('.offset(','[')
    data = data.replace('(isize)', '(usize)')
    data = data[:-1] + ']'
    print 'DUN',data
    return data
def desubarrayfun(match):
    data = match.group()
    data = data.replace('.offset(','[')
    data = data.replace('(isize)', '(usize)')
    data = data[:-1] + '..]'
    return data

with open(sys.argv[1]) as f:
    ret = p.sub(subfun, f.read())
    ret = p2.sub(dereffun, ret)
    ret = psubarray.sub(desubarrayfun, ret)
    ret = p2.sub(dereffun, ret)
    ret = psubarray.sub(desubarrayfun, ret)
    ret = ret.replace('i32 as (usize)', 'usize')
    ret = ret.replace('i32 as (u32)', 'u32')
    sys.stdout.write(ret)

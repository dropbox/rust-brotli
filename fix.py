import re
p = re.compile("""fn[^(]+[(]((([^),]*),)*)(([^),]*)[)])""")
def subfun(v):
    print 'SUBBING',v.group()
    return v.group().lower()
print p.sub(subfun, 'fn MYFUNC (first argument, second argument, third argument, last argument) { body of the \nfunction}  fn OTHERFUNC()')

print p.split('fn MYFUNC (only argument)')
print p.split('fn MYFUNC ()')

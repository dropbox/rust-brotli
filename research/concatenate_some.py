import sys
import os
import random as insecure_random
import subprocess
import hashlib
import traceback

def get_goal():
    ret = insecure_random.randrange(1,16)
    if ret == 15:
        ret = insecure_random.randrange(15,64)
    return ret

def shasum(work):
    sha = hashlib.sha256()
    for file in work:
        with open(file) as ff:
            while True:
                r = ff.read(65536)
                if r:
                    sha.update(r)
                else:
                    break
    return sha.digest()
def file_size(filename):
    return os.stat(filename).st_size
uncatable_stats = 0
catable_stats = 0
def process(work, brotli, cat, prefix):
    sys.stdout.flush()
    global uncatable_stats
    global catable_stats
    work = [filename for filename in work if filename.find(".jpg") != -1 or filename.find(".avi") != -1 or filename.find(".mp4") != -1 or filename.find(".mov") != -1 or insecure_random.randrange(1,17) == 16]
    if len(work) == 0:
        return
    try:
        fullsum = shasum(work)
    except Exception:
        traceback.print_exc()
        print 'ok early exit'
        return
    quality = "-q" + str(insecure_random.randrange(2,12 if len(work) < 8 else 10))
    if insecure_random.randrange(0,16) == 0:
        quality = '-q9.5'
    append = insecure_random.randrange(0,2) == 0
    frivolous_procs = []
    procs = []
    print 'processing',work,'at',quality,append
    for index, filename in enumerate(work):
        thread_count = insecure_random.randrange(1,17)
        if quality > 8 and thread_count == 1:
            thread_count = 2
        magic = insecure_random.randrange(0,2) == 0
        buffer_size = insecure_random.randrange(1,18)
        if buffer_size == 17:
            buffer_size = 65536
        elif buffer_size > 3:
            buffer_size *= 1024
        bs = "-bs" + str(buffer_size)
        args = [brotli, "-c", bs, quality]
        if magic:
            args.append("-magic")
        frivolous_procs.append(subprocess.Popen(args + [filename, prefix + quality+"-" + str(index)+".compressed"]))
        if append and index == 0:
            args.append("-appendable")
        else:
            args.append("-catable")
        args.append('-j' + str(thread_count))
        args.append(filename)
        args.append(prefix + quality+"-" + str(index)+".br")
        procs.append(subprocess.Popen(args))
    for index, proc in enumerate(procs):
        ret = proc.wait()
        if ret:
            print 'failure at ' + work[index],quality,append, bs
        assert not ret
    buffer_size = insecure_random.randrange(1,18)
    if buffer_size == 17:
        buffer_size = 65536
    elif buffer_size > 3:
        buffer_size *= 1024
    bs = "-bs" + str(buffer_size)
    args = [cat, bs]
    for index, filename in enumerate(work):
        args.append(prefix + quality+"-" + str(index)+".br")
    stdout, _stderr = subprocess.Popen(args, stdout=subprocess.PIPE).communicate()
    with open(prefix+".br", 'w') as f:
        f.write(stdout)
    procs[0] = subprocess.Popen([brotli, prefix +'.br', prefix])
    ret = procs[0].wait()
    if ret:
        print 'failure at ',work,quality,append
    assert not ret
    rtsum = shasum([prefix])
    if rtsum != fullsum:
        print 'failure at ',work,quality,append
    assert rtsum == fullsum
    print 'ok',rtsum.encode('hex')
    for (index,proc) in enumerate(frivolous_procs):
        ret = proc.wait()
        assert not ret
        uncatable_stats += file_size(prefix + quality+"-" + str(index)+".compressed")
        
    catable_stats += len(stdout)
    print uncatable_stats,'/',catable_stats,(10000*uncatable_stats/catable_stats)/100.0
    try:
        for index, filename in enumerate(work):
            os.remove(prefix + quality+"-" + str(index)+".br")
            os.remove(prefix + quality+"-" + str(index)+".compressed")
        os.remove(prefix + ".br")
        os.remove(prefix)
    except Exception:
        traceback.print_exc()
    
def main():
    start = sys.argv[1]
    brotli = sys.argv[2]
    catbrotli = sys.argv[3]
    goal = get_goal()
    work =[]
    prefix="/tmp/cat-" + os.urandom(16).encode('hex')
    for root, dirnames, filenames in os.walk(start):
        for filename in filenames:
            try:
                if file_size(os.path.join(root,filename)):
                    work.append(os.path.join(root,filename))
            except Exception:
                continue
            if len(work) >= goal:
                goal = get_goal()
                process(work, brotli, catbrotli, prefix)
                work = []
    if len(work):
        process(work, brotli, catbrotli,  prefix)
if __name__=="__main__":
    main()


from random import shuffle

def gen(lis):
    if not 0 in lis:
        return lis
    n = lis.index(0)
    l = [1,2,3,4,5,6,7,8,9]
    shuffle(l)
    for i in l:
        fail = False
        for k in range(81):
            if lis[k] == i:
                if k % 9 == n % 9 or k // 9 == n // 9 or (k // 27 == n // 27 and (k // 3) % 3 == (n // 3) % 3):
                    fail = True
                    break
        if fail:
            continue
        x = gen(lis[:n] + [i] + lis[n+1:])
        if not x == False:
            return x
    return False

def generate():
    return gen([0]*81)

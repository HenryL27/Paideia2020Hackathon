from random import shuffle
from copy import copy

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

def solvexcep(lis, g):
    if not 0 in lis:
        if lis == g:
            return False
        return lis
    n = lis.index(0)
    for i in range(1,10):
        fail = False
        for k in range(81):
            if lis[k] == i:
                if k % 9 == n % 9 or k // 9 == n // 9 or (k // 27 == n // 27 and (k // 3) % 3 == (n // 3) % 3):
                    fail = True
                    break
        if fail:
            continue
        x = solvexcep(lis[:n] + [i] + lis[n+1:], g)
        if not x == False:
            return x
    return False


def generate():
    grid = gen([0]*81)
    og = copy(grid)
    z = list(range(81))
    shuffle(z)
    x = 0
    for i in z:
        if not grid[i] == 0:
            if solvexcep(grid[:i] + [0] + grid[i+1:], og) == False:
                grid = grid[:i] + [0] + grid[i+1:]
        x += 1
        print(str(x) + " " + str(grid.count(0)))
    return grid

s = generate()
for i in range(9):
    print(s[9*i:9*(i+1)])

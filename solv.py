def solve_sudoku(lis):
    if not 0 in lis: # no empty spaces
        return lis
    n = lis.index(0) # first empty space
    for i in range(1,10): # ranging throught possible numbers
        fail = False # i does not work for n
        for k in range(81): # over each element in the array
            if lis[k] == i:
                # test for a conflict in row and column and square
                if k % 9 == n % 9 or k // 9 == n // 9 or (k // 27 == n // 27 and (k // 3) % 3 == (n // 3) % 3):
                    fail = True
                    break
        if fail:
            continue
        x = solve_sudoku(lis[:n] + [i] + lis[n+1:]) # solve on the updated list
        if not x == False:
            return x
    return False

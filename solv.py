import random


def solve_sudoku(lis):
    if not 0 in lis:  # no empty spaces
        return lis
    n = lis.index(0)  # first empty space
    numbers_to_range_over = list(range(1, 10))
    random.shuffle(numbers_to_range_over)
    for i in numbers_to_range_over:  # ranging throught possible numbers
        fail = False  # i does not work for n
        for k in range(81):  # over each element in the array
            if lis[k] == i:
                # test for a conflict in row and column and square
                if k % 9 == n % 9 or k // 9 == n // 9 or (
                        k // 27 == n // 27 and (k // 3) % 3 == (n // 3) % 3):
                    fail = True
                    break
        if fail:
            continue
        x = solve_sudoku(lis[:n] + [i] +
                         lis[n + 1:])  # solve on the updated list
        if not x == False:
            return x
    return False


def generate_sudoku():
    empty_board = [0] * 81
    return solve_sudoku(empty_board)


def display_sudoku(sudoku):
    print("+" + "-" * 20 + "+")
    for i in range(0, 9):
        print("|", end="")
        for j in range(0, 9):
            print(str(sudoku[i * 9 + j]) + " ", end="")
            if (j + 1) % 3 == 0:
                print("|", end="")
        if (i + 1) % 3 == 0 and i != 8:
            print("\n|" + "-" * 20 + "|", end="")
        print("", end="\n")
    print("+" + "-" * 20 + "+", end="\n")


display_sudoku(generate_sudoku())

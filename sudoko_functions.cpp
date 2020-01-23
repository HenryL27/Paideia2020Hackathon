#include <vector>

int sudoku_board_access(const std::vector<int> &sudoku, int row, int column)
{
  return sudoku.at(9 * (row - 1) + column);
}

std::vector<int> find_possible_options(const std::vector<int> sudoku, int row,
                                       int column)
{
  std::vector<int> out{1, 2, 3, 4, 5, 6, 7, 8, 9};
  for (int i = 1; i <= 9; i++) {
    if
      out.sudoku_board_access(sudoku, column, i)
  }
}

int main() { return 0; }

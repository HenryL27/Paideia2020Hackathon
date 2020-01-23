use rand::{seq::SliceRandom, thread_rng};

fn main() {
    println!("randomly generated sudoku board");
    print_sudoku(generate_sudoku());
}

type Sudoku = Vec<Option<u8>>;

fn solve_sodoku(board: Sudoku) -> Option<Sudoku> {
    let mut first_empty: usize = board.len();
    for (i, x) in board.iter().enumerate() {
        if x.is_none() {
            first_empty = i;
            break;
        }
    }
    if first_empty == board.len() {
        return Some(board);
    }
    let mut possibles: Vec<u8> = (1..10).collect();
    possibles.shuffle(&mut thread_rng());
    for i in possibles {
        let mut fail = false;
        for k in 0..81 {
            if board[k] == Some(i) {
                if k % 9 == first_empty % 9
                    || k / 9 == first_empty / 9
                    || (k / 27 == first_empty / 27 && (k / 3) % 3 == (first_empty / 3) % 3)
                {
                    fail = true;
                    break;
                }
            }
        }
        if fail {
            continue;
        }

        let mut x = board.clone();
        x[first_empty] = Some(i);
        let solved = solve_sodoku(x);
        if solved.is_some() {
            return solved;
        }
    }
    return None;
}

fn generate_sudoku() -> Sudoku {
    solve_sodoku(vec![None; 81]).unwrap()
}

fn print_sudoku(board: Sudoku) {
    println!("+{}+", "-".repeat(20));
    for i in 0..9 {
        print!("|");
        for j in 0..9 {
            match board[i * 9 + j] {
                Some(x) => print!("{:#?} ", x),
                None => print!("_"),
            };
            if (j + 1) % 3 == 0 {
                print!("|")
            }
        }
        if (i + 1) % 3 == 0 && i != 8 {
            print!("\n|{}|", "-".repeat(20));
        }
        print!("\n");
    }
    println!("+{}+", "-".repeat(20));
}

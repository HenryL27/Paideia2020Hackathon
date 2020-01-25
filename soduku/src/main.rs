use rand::{seq::SliceRandom, thread_rng, Rng};

fn main() {
    println!("randomly generated sudoku board");
    print_sudoku(generate_puzzle());
}

type Sudoku = Vec<Option<u8>>;

// Just tells whether two boards are identical
fn matching(board1: Sudoku, board2: Sudoku) -> bool {
    for i in 0..81 {
        if board1[i] != board2[i]{
            return false;
        }
    }
    return true;
}

// Function to remove a square pair from the thing
fn remove_square(board: Sudoku, ans: Sudoku) -> Sudoku {
    let mut rng = thread_rng();
    let rem = rng.gen_range(0, 81);
    board[rem] = None;
    board[81-rem] = None;
    if matching(solve_sodoku(board), ans){
        return board;
    } else {
        return None;
    }
}

// Function to create a board and then remove from it
fn generate_puzzle() -> Sudoku{
    let puzzle = generate_sudoku();
    let ans = puzzle.clone();
    while true {
        let p = remove_square(puzzle, ans);
        if p.is_none(){ return puzzle; }
        puzzle = p;
    }
}

// Solves a sudoku
fn solve_sodoku(board: Sudoku) -> Option<Sudoku> {
    // Find the first empty square
    let mut first_empty: usize = board.len();
    for (i, x) in board.iter().enumerate() {
        if x.is_none() {
            first_empty = i;
            break;
        }
    }
    // If nothing empty, return board
    if first_empty == board.len() {
        return Some(board);
    }
    // Plug in numbers for that empty guy
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
        // Recurive call
        let solved = solve_sodoku(x);
        if solved.is_some() {
            return solved;
        }
    }
    // Can't solve -> None
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

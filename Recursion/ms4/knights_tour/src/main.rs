use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

const BOARD_SIZE: i32 = INUM_ROWS * INUM_COLS;
// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn move_knight(
    board: &[[i32; NUM_COLS]; NUM_ROWS],
    row: i32,
    col: i32,
    offset: &[i32; 2],
) -> Option<(usize, usize)> {
    let new_row = row + offset[0];
    let new_col = col + offset[1];

    if (0..INUM_ROWS).contains(&new_row)
        && (0..INUM_COLS).contains(&new_col)
        && board[new_row as usize][new_col as usize] == UNVISITED
    {
        return Some((new_row as usize, new_col as usize));
    }
    None
}
// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &mut [[i32; 2]; 8], // 8 possible moves, 2 coordinates each.
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
) -> bool {
    match num_visited {
        // If we have visited all squares, we are done.
        BOARD_SIZE if !REQUIRE_CLOSED_TOUR => true,
        // If we have visited all squares and we are back at the start, we are done.
        BOARD_SIZE if REQUIRE_CLOSED_TOUR => {
            if offsets.iter().any(|&offset| {
                if let Some((row, col)) = move_knight(board, cur_row, cur_col, &offset) {
                    if board[row][col] == 0 {
                        return true;
                    }
                    false
                } else {
                    false
                }
            }) {
                return true;
            }
            false
        }
        _ => {
            board[cur_row as usize][cur_col as usize] = num_visited;
            let mut next_fields = offsets
                .iter()
                .map(|offset| move_knight(board, cur_row, cur_col, &offset))
                .filter(|field| field.is_some())
                .map(|opt_field| opt_field.unwrap());

            next_fields.any(|(row, col)| {
                if find_tour(board, offsets, row as i32, col as i32, num_visited + 1) {
                    return true;
                }
                board[row][col] = UNVISITED;
                false
            })
            // iterate over the possible moves
        }
    }
}

fn dump_board(board: &mut [[i32; NUM_COLS]; NUM_ROWS]) {
    for row in board.iter() {
        for field in row.iter() {
            print!("{:3} ", field);
        }
        println!();
    }
}

fn main() {
    // Initialize the vector of move offsets.
    let mut offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &mut offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

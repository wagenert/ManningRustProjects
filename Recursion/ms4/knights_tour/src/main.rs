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

fn find_fields(
    board: &[[i32; NUM_COLS]; NUM_ROWS],
    row: &i32,
    col: &i32,
    offset: &[i32; 2],
    target_value: &i32,
) -> Option<(usize, usize)> {
    let new_row = row + offset[0];
    let new_col = col + offset[1];

    if (0..INUM_ROWS).contains(&new_row)
        && (0..INUM_COLS).contains(&new_col)
        && board[new_row as usize][new_col as usize] == *target_value
    {
        return Some((new_row as usize, new_col as usize));
    }
    None
}

fn find_possible_moves(
    board: &[[i32; NUM_COLS]; NUM_ROWS],
    row: &i32,
    col: &i32,
    offset: &[i32; 2],
) -> Option<(usize, usize)> {
    find_fields(board, row, col, offset, &UNVISITED)
}

fn is_end_of_cyclic_tour(
    board: &[[i32; NUM_COLS]; NUM_ROWS],
    row: &i32,
    col: &i32,
    offset: &[i32; 2],
) -> bool {
    find_fields(board, row, col, offset, &0).is_some()
}

fn move_knight_warnsdorff(
    board: &[[i32; NUM_COLS]; NUM_ROWS],
    row: i32,
    col: i32,
    offsets: &[[i32; 2]; 8],
) -> Option<Vec<(usize, usize)>> {
    // get all possible valid moves
    let mut next_fields: Vec<(usize, usize)> = offsets
        .iter()
        .filter_map(|offset| find_possible_moves(board, &row, &col, offset))
        .collect();

    // if there are no valid moves return None
    if next_fields.is_empty() {
        return None;
    }

    // sort the moves by the number of possible moves from that field
    next_fields.sort_by(|a, b| {
        let a = offsets
            .iter()
            .filter_map(|offset| find_possible_moves(board, &(a.0 as i32), &(a.1 as i32), offset))
            .count();
        let b = offsets
            .iter()
            .filter_map(|offset| find_possible_moves(board, &(b.0 as i32), &(b.1 as i32), offset))
            .count();
        a.cmp(&b)
    });

    Some(next_fields)
}

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &[[i32; 2]; 8], // 8 possible moves, 2 coordinates each.
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
) -> bool {
    match num_visited {
        // We have visited all squares. Done.
        BOARD_SIZE if !REQUIRE_CLOSED_TOUR => true,
        // Check if the first square is reachable from the current position.
        BOARD_SIZE if REQUIRE_CLOSED_TOUR => offsets
            .iter()
            .any(|&offset| is_end_of_cyclic_tour(board, &cur_row, &cur_col, &offset)),
        _ => {
            // Try to identify next square
            if let Some(next_fields) = move_knight_warnsdorff(board, cur_row, cur_col, offsets) {
                next_fields.iter().any(|(row, col)| {
                    // Set the possible next square to the current number of visited squares and descent to the next level.
                    board[*row][*col] = num_visited;
                    // If that path delivers a successfull solution return true.
                    if find_tour(board, offsets, *row as i32, *col as i32, num_visited + 1) {
                        true
                    } else {
                        // Trace back by setting the square to UNVISITED.
                        board[*row][*col] = UNVISITED;
                        false
                    }
                })
            } else {
                // No more squares to visit. Trace back.
                false
            }
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
    let offsets = [
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
    let starting_position = (5, 5);
    // Start at board[0][0].
    board[starting_position.0 as usize][starting_position.1 as usize] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(
        &mut board,
        &offsets,
        starting_position.0,
        starting_position.1,
        1,
    );
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

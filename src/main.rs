
// A sample program that creates a 9x9 board with 10 mines

use rand::Rng;

// generate a random number between 0 and 8
fn random_number() -> u32 {
    rand::thread_rng().gen_range(0, 8)
}

// Generate a random mine location
fn random_mine_location() -> (u32, u32) {
    (random_number(), random_number())
}

// Create a 9x9 board with 10 mines
fn create_board() -> [[u32; 9]; 9] {
    let mut board = [[0; 9]; 9];
    let mut mine_locations = Vec::new();

    // Generate 10 random mine locations
    while mine_locations.len() < 10 {
        let mine_location = random_mine_location();
        if !mine_locations.contains(&mine_location) {
            mine_locations.push(mine_location);
        }
    }

    // Add mines to the board
    for &(x, y) in mine_locations.iter() {
        board[x as usize][y as usize] = 9;
    }

    // Calculate the number of adjacent mines for each cell
    for x in 0..9 {
        for y in 0..9 {
            if board[x as usize][y as usize] != 9 {
                board[x as usize][y as usize] = adjacent_mines(x, y, &board);
            }
        }
    }

    board
}

// Get the number of adjacent mines
fn adjacent_mines(x: u32, y: u32, board: &[[u32; 9]; 9]) -> u32 {
    let mut adjacent_mines = 0;

    // Check the adjacent cells
    for a in x.saturating_sub(1)..=x.min(8) {
        for b in y.saturating_sub(1)..=y.min(8) {
            if board[a as usize][b as usize] == 9 {
                adjacent_mines += 1;
            }
        }
    }

    adjacent_mines
}

// Print the board
fn print_board(board: [[u32; 9]; 9]) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let board = create_board();
    print_board(board);
    let mut i = 10;
    i = i+1;
    while  i<11 {
    }
}

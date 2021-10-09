const COL: usize = 10;
const ROW: usize = 15;

fn main() {
    // Array is X, Y but matrix is Row, Column
    // create the main board 2d-array
    let mut board: [[i8; COL]; ROW] = [[0; COL]; ROW];
    print(&board);

    // create a boarder around the board as walls
    // ----------------------------------------
    // in this methods I have to iterate over all cell one by one and I think it's not so optimize!
    // for (row, row_value) in board.iter_mut().enumerate() {
    //     for (col, col_value) in row_value.iter_mut().enumerate() {
    //         if row == 0 || row == ROW-1 || col == 0 || col == COL-1 {
    //             *col_value = -1;
    //         } else {
    //             *col_value = 0; // we can clear others board cell with 0
    //         }
    //     }
    // }
    // ----------------------------------------
    // loop from 0 to ROW but just change the first and last columns value
    // loop from 0 to COL but just change the first and last rows value
    for y in 0..ROW {
        board[y][0] = -1;       // first column
        board[y][COL - 1] = -1; // last column
    }
    // skip first and last column because they are filled in above loop
    for x in 1..COL-1 {
        board[0][x] = -1;       // first row
        board[ROW - 1][x] = -1; // last row
    }
    print(&board);

    
    
}

// print just the raw value of each array cell
fn print(board: &[[i8; COL]; ROW]) {
    for y in 0..ROW {
        for x in 0..COL {
            // ROW, COL == y, x
            print!("{:2}", board[y][x]);
        }
        print!("\n");
    }
}
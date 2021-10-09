const COL: usize = 10;
const ROW: usize = 15;

#[derive(PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self { 
            x: 2,
            y: 5,
        }
    }
}

fn main() {
    // Array is X, Y but matrix is Row, Column
    // create the main board 2d-array
    let mut board: [[i8; COL]; ROW] = [[0; COL]; ROW];
    print_dbg(&board);

    // snake is just one-dimensional dynamic array(vector) containing points of each part of snake
    let mut snake: Vec<Point> = vec![Point::default()];

    create_wall(&mut board);
    print_dbg(&board);

    // for showing snake I can place the snake vector into board by adding it or keep them separate and just combine them on render method!?
    // place_snake(&mut board, &snake);
    // print_dbg(&board);
    // in second method don't need to add snake into the board just need change on print/render method to render board and snake separately
    print_with_snake_dbg(&board, &snake);
    
}

// create wall around the board
fn create_wall(board: &mut [[i8; COL]; ROW]) {
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
}

// print just the raw value of each array cell
fn print_dbg(board: &[[i8; COL]; ROW]) {
    for y in 0..ROW {
        for x in 0..COL {
            // ROW, COL == y, x
            print!("{:2}", board[y][x]);
        }
        print!("\n");
    }
}

// place & combine the snake into the board
fn place_snake(board: &mut [[i8; COL]; ROW], snake: &Vec<Point>) {
    // I start with first method because I think it would be easy maybe!?
    for (index, point) in snake.iter().enumerate() {
        board[point.y][point.x] = if index == 0 { 2 } else { 1 };
    }
}

// print the raw value of each array cell with snake points on it
fn print_with_snake_dbg(board: &[[i8; COL]; ROW], snake: &Vec<Point>) {
    // this is method 2 for rendering snake separately from board
    let mut c;

    for y in 0..ROW {
        for x in 0..COL {
            // ROW, COL == y, x
            // on each loop find any point with the same position as current board cell and if check for the head of snake too
            c = if let Some(point) = snake.into_iter().find(|p| p.x == x && p.y == y)  {
                if snake.first().unwrap() == point { 2 } else { 1 }
            } else {
                board[y][x]
            };

            print!("{:2}", c);

        }
        print!("\n");
    }
}
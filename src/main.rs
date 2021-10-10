use rand::thread_rng;
use rand::Rng;

const COLS: usize = 10;
const ROWS: usize = 15;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
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
    let mut board: [[i8; COLS]; ROWS] = [[0; COLS]; ROWS];
    print_dbg(&board);

    // snake is just one-dimensional dynamic array(vector) containing points of each part of snake
    let mut snake: Vec<Point> = vec![Point::default()];

    let mut food: Option<Point> = None;

    create_wall(&mut board);
    print_dbg(&board);

    // for showing snake I can place the snake vector into board by adding it or keep them separate and just combine them on render method!?
    // place_snake(&mut board, &snake);
    // print_dbg(&board);
    // in second method don't need to add snake into the board just need change on print/render method to render board and snake separately
    print_with_snake_dbg(&board, &snake);

    generate_food(&mut food);
    // place_food(&mut board, &food);
    // print_dbg(&board);
    print_with_food_dbg(&board, &food);


}

fn generate_food(food: &mut Option<Point>) {
    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(1..COLS-1);
    let y: usize = rng.gen_range(1..ROWS-1);

    if let Some(f) = food {
        f.set(x, y);
    } else {
        *food = Some(Point::new(x, y));
    }
}

// create wall around the board
fn create_wall(board: &mut [[i8; COLS]; ROWS]) {
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
    for y in 0..ROWS {
        board[y][0] = -1;       // first column
        board[y][COLS - 1] = -1; // last column
    }
    // skip first and last column because they are filled in above loop
    for x in 1..COLS-1 {
        board[0][x] = -1;       // first row
        board[ROWS - 1][x] = -1; // last row
    }
}

// print just the raw value of each array cell
fn print_dbg(board: &[[i8; COLS]; ROWS]) {
    for y in 0..ROWS {
        for x in 0..COLS {
            // ROW, COL == y, x
            print!("{:2}", board[y][x]);
        }
        print!("\n");
    }
}

// place & combine the snake into the board
fn place_snake(board: &mut [[i8; COLS]; ROWS], snake: &Vec<Point>) {
    // I start with first method because I think it would be easy maybe!?
    for (index, point) in snake.iter().enumerate() {
        board[point.y][point.x] = if index == 0 { 2 } else { 1 };
    }
}

// print the raw value of each array cell with snake points on it
fn print_with_snake_dbg(board: &[[i8; COLS]; ROWS], snake: &Vec<Point>) {
    // this is method 2 for rendering snake separately from board
    let mut c;

    for y in 0..ROWS {
        for x in 0..COLS {
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

// place the food into the board by add it to board array
fn place_food(board: &mut [[i8; COLS]; ROWS], food: &Option<Point>) {
    if let Some(point) = food {
        board[point.y][point.x] = 3;
    }
}

// print the raw value of each array cell with food points on it
fn print_with_food_dbg(board: &[[i8; COLS]; ROWS], food: &Option<Point>) {
    let mut c;
    let point = &food.unwrap();

    for y in 0..ROWS {
        for x in 0..COLS {
            // ROW, COL == y, x
            // on each loop find any point with the same position as current board cell and if check for the head of snake too
            c = if point.x == y && point.y == x {
                3
            } else {
                board[y][x]
            };

            print!("{:2}", c);

        }
        print!("\n");
    }
}
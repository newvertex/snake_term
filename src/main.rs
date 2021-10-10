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

    // snake is just one-dimensional dynamic array(vector) containing points of each part of snake
    let mut snake: Vec<Point> = vec![Point::default()];
    let mut food: Option<Point> = None;

    create_wall(&mut board);
    place_snake(&mut board, &snake);
    generate_food(&mut board, &mut food);
    print_dbg(&board);

}

fn generate_food(board: &mut [[i8; COLS]; ROWS], food: &mut Option<Point>) {
    let mut rng = rand::thread_rng();
    loop {
        let x: usize = rng.gen_range(1..COLS-1);
        let y: usize = rng.gen_range(1..ROWS-1);
        if board[y][x] != 0 {
            continue;
        }

        if let Some(f) = food {
            f.set(x, y);
        } else {
            *food = Some(Point::new(x, y));
        }

        board[y][x] = 3;
        break;
    }
}

// create wall around the board
fn create_wall(board: &mut [[i8; COLS]; ROWS]) {
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

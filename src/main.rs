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

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Snake {
    body: Vec<Point>,
    dir: Direction,
}

impl Snake {
    fn new(body: Vec<Point>, dir: Direction) -> Self {
        Self {
            body,
            dir,
        }
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            body: vec![Point::new(2, 5), Point::new(2, 4)],
            dir: Direction::Down,
        }
    }
}

enum Shape {
    Empty = 0,
    Wall = -1,
    Body = 1,
    Head = 2,
    Food = 3,
}

fn main() {
    // Array is X, Y but matrix is Row, Column
    let (level, mut snake) = load_level();

    let mut food: Option<Point> = None;
    generate_food(&mut food, &level, &snake);

    println!("{}", render(&level, &snake, &food));

}

// load default level for now
fn load_level() -> (Vec<Vec<i8>>, Snake) {
    // get the width, height from the map file
    let width = COLS;
    let height = ROWS;

    // get snake start position and create it
    let snake = Snake::default();

    // load the maps file and fill the level vector with it's value 
    let mut level = vec![vec![Shape::Empty as i8; width]; height];

    for row in 0..height {
        level[row][0] = Shape::Wall as i8;
        level[row][width - 1] = Shape::Wall as i8;
    }
    for column in 1..(width - 1) {
        level[0][column] = Shape::Wall as i8;
        level[height - 1][column] = Shape::Wall as i8;
    }

    (level, snake)
}

fn generate_food(food: &mut Option<Point>, level: &Vec<Vec<i8>>, snake: &Snake) {
    let mut rng = rand::thread_rng();
    let width = level[0].len();
    let height = level.len();

    loop {
        let x: usize = rng.gen_range(0..width);
        let y: usize = rng.gen_range(0..height);
        if level[y][x] != 0 || snake.body.iter().any(|p| p.x == x && p.y == y) {
            continue;
        }

        if let Some(f) = food {
            f.set(x, y);
        } else {
            *food = Some(Point::new(x, y));
        }

        break;
    }
}

// render board to string
fn render(level: &Vec<Vec<i8>>, snake: &Snake, food: &Option<Point>) -> String {
    // create a backbuffer and put everything together before start rendering
    let mut back_buffer = level.clone();

    for point in &snake.body {
        back_buffer[point.y][point.x] = if snake.body.first().unwrap() == point {
             Shape::Head as i8
        } else {
            Shape::Body as i8
        };
    }

    if let Some(point) = food {
        back_buffer[point.y][point.x] = Shape::Food as i8;
    }

    let mut output_buffer = "".to_string();
    let mut c;
    for row in &back_buffer {
        for col in row {
            let shape: Shape = unsafe { std::mem::transmute(*col) };
            c = match shape {
                Shape::Empty => '_',
                Shape::Wall => '$',
                Shape::Body => '#',
                Shape::Head => '@',
                Shape::Food => '%',
                _ => '~'
            };

            output_buffer.push(' ');
            output_buffer.push(c);
        }
        output_buffer.push('\n');
    }

    output_buffer
}
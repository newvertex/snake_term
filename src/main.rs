use rand::Rng;

const COLS: usize = 10;
const ROWS: usize = 15;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

    fn add(&mut self, x: usize, y: usize) {
        self.x += x;
        self.y += y;
    }

    fn sub(&mut self, x: usize, y: usize) {
        self.x -= x;
        self.y -= y;
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
            body: vec![Point::new(2, 5), Point::new(2, 4), Point::new(2, 3)],
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
    let mut cx = 0;
    loop {
        print!("\x1B[2J\x1B[1;1H"); // x1B => 27 ac char => escape, clear and move cursor to 1, 1
        println!("{}", render(&level, &snake, &food));

        // make a copy of snake head and change it's position based on direction
        let mut snake_head = snake.body[0];
        match snake.dir {
            Direction::Right => snake_head.add(1, 0),
            Direction::Down => snake_head.add(0, 1),
            Direction::Left => snake_head.sub(1, 0),
            Direction::Up => snake_head.sub(0, 1),
        }
        // insert new head as first element and pop the last element
        snake.body.insert(0, snake_head);

        // if the next position is not the food then remove the tail else just remove the food and regenerate it, let's the snake grow
        if snake_head != food.unwrap()  {
            snake.body.pop();
        } else {
            // TODO: add score
            generate_food(&mut food, &level, &snake);
        }

        // check snake new head have any collision with it self if true then game over
        if snake.body[1..].contains(&snake_head) {
            println!("Game Over!");
            break;
        }

        // check the new head position inside the map to see if there is a wall or it's empty cell, if it's the wall then game over
        if level[snake_head.y][snake_head.x] == Shape::Wall as i8 {
            println!("Hit the wall Game Over!");
            break;
        }

        // just for test
        std::thread::sleep(std::time::Duration::from_secs(1));

        cx += 1;
        if cx == 3 {
            snake.dir = Direction::Right; 
        }
        if cx == 6 {
            snake.dir = Direction::Up; 
        }
        if cx == 9 {
            snake.dir = Direction::Left; 
        }
        if cx == 12 {
            snake.dir = Direction::Down; 
            cx = 0;
        }
    }

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
            };

            output_buffer.push(' ');
            output_buffer.push(c);
        }
        output_buffer.push('\n');
    }

    output_buffer
}
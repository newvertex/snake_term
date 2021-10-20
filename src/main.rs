use std::collections::HashMap;
use std::time::Duration;

use rand::Rng;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{Event, KeyCode, KeyEvent, poll, read};

use quad_snd::{AudioContext, Sound};

const COLS: usize = 10;
const ROWS: usize = 15;
const ASSETS: &str = "assets/";


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

#[derive(PartialEq, Eq, Clone, Copy)]
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

    fn move_to(&mut self, dir: Direction) {
        use Direction::*;

        self.dir = match dir {
            Right if self.dir != Left => Right,
            Down if self.dir != Up => Down,
            Left if self.dir != Right => Left,
            Up if self.dir != Down => Up,
            _ => self.dir,
        };
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

struct Audio {
    ctx: AudioContext,
    sounds: HashMap<String, Sound>,
}

impl Audio {
    fn new() -> Self {
        Self {
            ctx: AudioContext::new(),
            sounds: HashMap::new(),
        }
    }

    fn add_sound(&mut self, file_names: &[&str]) {
        for file in file_names {
            let file_path = ASSETS.to_string() + file + ".ogg";
            let bytes = std::fs::read(file_path).unwrap();
            let sound = Sound::load(&mut self.ctx, bytes.as_slice());
            self.sounds.insert(file.to_string(), sound);
        }
    }

    fn play_once(&mut self, sound_name: &str) {
        let snd = self.sounds.get_mut(sound_name).unwrap();
        snd.stop(&mut self.ctx);
        snd.play(&mut self.ctx, Default::default());
    }

    fn play_loop(&mut self, sound_name: &str) {
        let snd = self.sounds.get_mut(sound_name).unwrap();
        snd.stop(&mut self.ctx);
        snd.play(&mut self.ctx, quad_snd::PlaySoundParams{ looped: true, volume: 1.0 });
    }

    fn stop(&mut self, sound_name: &str) {
        self.sounds.get_mut(sound_name).unwrap().stop(&mut self.ctx);
    }

    fn stop_all(&mut self) {
        for sound in self.sounds.values_mut() {
            sound.stop(&mut self.ctx);
        }
    }
}

fn main() -> crossterm::Result<()> {
    let (level, mut snake) = load_level();
    let mut food: Option<Point> = None;
    generate_food(&mut food, &level, &snake);

    let mut audio = Audio::new();
    audio.add_sound(&["snake_hit", "snake_grow", "music"]);
    audio.play_loop("music");
    
    enable_raw_mode()?;

    loop {
        print!("\x1B[2J\x1B[1;1H"); // x1B => 27 ac char => escape, clear and move cursor to 1, 1
        println!("{}", render(&level, &snake, &food));

        handle_input(&mut snake)?;

        if !update(&level, &mut snake, &mut food, &mut audio) {
            break;
        }
    }
    
    audio.stop("music");
    
    disable_raw_mode()
}

fn handle_input(snake: &mut Snake) -> crossterm::Result<()> {
    if poll(Duration::from_millis(500))? {
        if let Event::Key(KeyEvent { code, .. }) = read()? {
            match code {
                KeyCode::Enter => todo!(),
                KeyCode::Left | KeyCode::Char('a') => snake.move_to(Direction::Left),
                KeyCode::Right | KeyCode::Char('d') => snake.move_to(Direction::Right),
                KeyCode::Up | KeyCode::Char('w') => snake.move_to(Direction::Up),
                KeyCode::Down | KeyCode::Char('s') => snake.move_to(Direction::Down),
                KeyCode::Esc => todo!(),
                _ => {}
            }
        }
    }

    Ok(())
}

fn update(level: &Vec<Vec<i8>>, snake: &mut Snake, food: &mut Option<Point>, audio: &mut Audio) -> bool {
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
        audio.play_once("snake_grow");
        generate_food(food, &level, &snake);
    }

    // check snake new head have any collision with it self if true then game over
    if snake.body[1..].contains(&snake_head) {
        audio.play_once("snake_hit");
        std::thread::sleep(Duration::from_millis(500));
        println!("Game Over!");
        return false;
    }

    // check the new head position inside the map to see if there is a wall or it's empty cell, if it's the wall then game over
    if level[snake_head.y][snake_head.x] == Shape::Wall as i8 {
        audio.play_once("snake_hit");
        std::thread::sleep(Duration::from_millis(500));
        println!("Hit the wall Game Over!");
        return false;
    }
   
    true
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
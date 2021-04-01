// use std::cmp::Ordering; //, convert::TryInto, rc::Rc};

// use ahash::AHashSet;
use rustc_hash::FxHashSet;
use std::{
    convert::TryInto,
    fmt::{self, Display, Write},
    hash::{Hash, Hasher},
};

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
struct Car {
    vertical: bool,
    length: i32,
    colour: u8,
    x: i32,
    y: i32,
}

impl Car {
    pub fn new(vertical: bool, length: i32, colour: u8, x: i32, y: i32) -> Self {
        Car {
            vertical,
            length,
            colour,
            x,
            y,
        }
    }
    pub fn is_victorious(&self) -> bool {
        self.colour == b'X' && self.x >= 4
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    cars: Vec<Car>,
    pub board_u8s: [[u8; 6]; 6],
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board_u8s == other.board_u8s
    }
}

impl Eq for Board {}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board_u8s.hash(state)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.board_u8s {
            for c in line {
                f.write_char(*c as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Board {
    pub fn from_path(board_path: &str) -> Self {
        let board_string =
            std::fs::read_to_string(board_path).expect("Error: could not read file. Panicking!");
        let mut cars: Vec<Car> = Vec::with_capacity(15); //largest board in test suite has only 15 colours / cars
        let mut colours: FxHashSet<u8> = FxHashSet::default(); //so we reserve that amount
        colours.insert(b'.');
        let u8s = str_to_u8s(&board_string);
        for (y, line) in board_string.lines().enumerate() {
            for (x, tile) in line.as_bytes().iter().enumerate() {
                if !colours.contains(tile) {
                    colours.insert(*tile);
                    cars.push(Car::new(
                        Self::is_vertical(u8s, x, y),
                        Self::get_length(u8s, x, y),
                        *tile,
                        x as i32,
                        y as i32,
                    ));
                }
            }
        }
        Board {
            cars,
            board_u8s: u8s,
        }
    }

    fn from_cars(cars: Vec<Car>) -> Self {
        let u8s = Self::gen_u8s(&cars);
        Board {
            board_u8s: u8s,
            cars,
        }
    }

    fn gen_u8s(cars: &[Car]) -> [[u8; 6]; 6] {
        let mut retval = [[b'.'; 6]; 6];

        for car in cars {
            if car.vertical {
                for i in 0..car.length + 1 {
                    retval[(car.y + i) as usize][car.x as usize] = car.colour;
                }
            } else {
                for i in 0..car.length + 1 {
                    retval[car.y as usize][(car.x + i) as usize] = car.colour;
                }
            }
        }

        retval
    }

    fn is_vertical(board_string: [[u8; 6]; 6], x: usize, y: usize) -> bool {
        y < 5 && board_string[y][x] == board_string[y + 1][x]
    }

    fn get_length(board_string: [[u8; 6]; 6], x: usize, y: usize) -> i32 {
        let colour = board_string[y][x];
        if Self::is_vertical(board_string, x, y) {
            if y < 4 && board_string[y + 2][x] == colour {
                2
            } else {
                1
            }
        } else if x < 4 && board_string[y][x + 2] == colour {
            2
        } else {
            1
        }
    }

    pub fn get_moves(&mut self) -> Vec<Self> {
        let mut carses = Vec::with_capacity(20);

        let cars = &mut self.cars as *mut Vec<Car>;

        // I think this is completely safe, actually. I never do anything too weird with memory.
        // the only reason I need unsafe is to replicate basically these lines of code:
        // car.x += 1;
        // cars.clone();
        // car.x -= 1;
        unsafe {
            for car in &mut *cars {
                if !car.vertical {
                    for i in 1..5 {
                        if car.x - i >= 0
                            && self.board_u8s[car.y as usize][(car.x - i) as usize] == b'.'
                        {
                            car.x -= i;
                            carses.push((*cars).clone());
                            car.x += i;
                        } else {
                            break;
                        }
                    }
                    for i in 1..5 {
                        if car.x + car.length + i < 6
                            && self.board_u8s[car.y as usize][(car.x + car.length + i) as usize]
                                == b'.'
                        {
                            car.x += i;
                            carses.push((*cars).clone());
                            car.x -= i;
                        } else {
                            break;
                        }
                    }
                } else {
                    for i in 1..5 {
                        if car.y - i >= 0
                            && self.board_u8s[(car.y - i) as usize][car.x as usize] == b'.'
                        {
                            car.y -= i;
                            carses.push((*cars).clone());
                            car.y += i;
                        } else {
                            break;
                        }
                    }
                    for i in 1..5 {
                        if car.y + car.length + i < 6
                            && self.board_u8s[(car.y + car.length + i) as usize][car.x as usize]
                                == b'.'
                        {
                            car.y += i;
                            carses.push((*cars).clone());
                            car.y -= i;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        carses.into_iter().map(Self::from_cars).collect()
    }

    pub fn is_solved(&self) -> bool {
        for car in &self.cars {
            if car.is_victorious() {
                return true;
            }
        }
        false
    }
}

fn str_to_u8s(board_string: &str) -> [[u8; 6]; 6] {
    let mut u8_array = [[b'0'; 6]; 6];
    let mut seperated_board = board_string.lines();

    for row in &mut u8_array {
        *row = seperated_board
            .next()
            .unwrap()
            .as_bytes()
            .try_into()
            .expect("Invalid board");
    }

    u8_array
}

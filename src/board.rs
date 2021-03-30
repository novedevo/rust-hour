use std::{cmp::Ordering, convert::TryInto, rc::Rc};

use ahash::AHashSet;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
struct Car {
    vertical: bool,
    length: i32,
    colour: char,
    x: i32,
    y: i32,
}

impl Car {
    pub fn new(vertical: bool, length: i32, colour: char, x: i32, y: i32) -> Self {
        Car {
            vertical,
            length,
            colour,
            x,
            y,
        }
    }
    pub fn is_victorious(&self) -> bool {
        self.colour == 'X' && self.x >= 4
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    cars: Vec<Car>,
    pub board_chars: [[char; 6]; 6],
    pub h: usize,
    pub g: usize,
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f())
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board_chars == other.board_chars
    }
}

impl Eq for Board {}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board_chars.hash(state)
    }
}

impl Board {
    pub fn from_str(board_path: &str) -> Self {
        let board_string = std::fs::read_to_string(board_path).unwrap();
        let mut cars: Vec<Car> = Vec::new();
        let mut colours: AHashSet<char> = AHashSet::new();
        colours.insert('.');
        let chars = str_to_chars(&board_string);
        for (y, line) in board_string.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if !colours.contains(&tile) {
                    colours.insert(tile);
                    cars.push(Car::new(
                        Self::is_vertical(chars, x, y),
                        Self::get_length(chars, x, y),
                        tile,
                        x as i32,
                        y as i32,
                    ));
                }
            }
        }
        Board {
            cars,
            board_chars: chars,
            g: 0,
            h: Self::gen_heuristic(chars),
        }
    }

    fn from_car_ref(cars: &Vec<Car>, parental_g: usize) -> Self {
        let chars = Self::gen_chars(cars);
        Board {
            board_chars: chars,
            cars: cars.clone(),
            g: parental_g + 1,
            h: Self::gen_heuristic(chars),
        }
    }

    fn from_cars(cars: Vec<Car>, parental_g: usize) -> Self {
        let chars = Self::gen_chars(&cars);
        Board {
            board_chars: chars,
            cars,
            g: parental_g + 1,
            h: Self::gen_heuristic(chars),
        }
    }

    pub fn to_str(&self) -> Vec<char> {
        let mut acc = vec![];
        for line in self.board_chars.iter() {
            for c in line {
                acc.push(*c);
            }
            acc.push('\n')
        }
        acc
    }

    fn gen_chars(cars: &[Car]) -> [[char; 6]; 6] {
        let mut retval = [['.'; 6]; 6];

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

    fn is_vertical(board_string: [[char; 6]; 6], x: usize, y: usize) -> bool {
        y < 5 && board_string[y][x] == board_string[y + 1][x]
    }

    fn get_length(board_string: [[char; 6]; 6], x: usize, y: usize) -> i32 {
        let colour = board_string[y][x];
        if Self::is_vertical(board_string, x, y) {
            if y < 4 && board_string[y + 2][x] == colour {
                2
            } else {
                1
            }
        } else {
            if x < 4 && board_string[y][x + 2] == colour {
                2
            } else {
                1
            }
        }
    }

    pub fn get_moves(&self) -> Vec<Self> {
        let mut moves = vec![];

        let cars = &mut self.cars.clone() as *mut Vec<Car>;
        
        // I think this is completely safe, actually. I never do anything too weird with memory.
        // the only reason I need unsafe is to replicate basically these lines of code:
        // car.x += 1;
        // cars.clone();
        // car.x -= 1;
        //
        unsafe {
            for car in &mut *cars {
                if !car.vertical {
                    for i in 1..5 {
                        if car.x - i >= 0
                            && self.board_chars[car.y as usize][(car.x - i) as usize] == '.'
                        {
                            car.x -= i;

                            let new_board = Board::from_cars((*cars).clone(), self.g);

                            moves.push(new_board);

                            car.x += i;

                            // moves.push(Self::add_to_moves(
                            //     car.x - i,
                            //     car.y,
                            //     car,
                            //     &self.cars,
                            //     self.g,
                            // ))
                        } else {
                            break;
                        }
                    }
                    for i in 1..5 {
                        if car.x + car.length + i < 6
                            && self.board_chars[car.y as usize][(car.x + car.length + i) as usize]
                                == '.'
                        {
                            car.x += i;

                            let new_board = Board::from_cars((*cars).clone(), self.g);

                            moves.push(new_board);

                            car.x -= i;
                            // moves.push(Self::add_to_moves(
                            //     car.x + i,
                            //     car.y,
                            //     car,
                            //     &self.cars,
                            //     self.g,
                            // ))
                        } else {
                            break;
                        }
                    }
                } else {
                    for i in 1..5 {
                        if car.y - i >= 0
                            && self.board_chars[(car.y - i) as usize][car.x as usize] == '.'
                        {
                            car.y -= i;

                            let new_board = Board::from_cars((*cars).clone(), self.g);

                            moves.push(new_board);

                            car.y += i;
                            // moves.push(Self::add_to_moves(
                            //     car.x,
                            //     car.y - i,
                            //     car,
                            //     &self.cars,
                            //     self.g,
                            // ))
                        } else {
                            break;
                        }
                    }
                    for i in 1..5 {
                        if car.y + car.length + i < 6
                            && self.board_chars[(car.y + car.length + i) as usize][car.x as usize]
                                == '.'
                        {
                            car.y += i;

                            let new_board = Board::from_cars((*cars).clone(), self.g);

                            moves.push(new_board);

                            car.y -= i;
                            // moves.push(Self::add_to_moves(
                            //     car.x,
                            //     car.y + i,
                            //     car,
                            //     &self.cars,
                            //     self.g,
                            // ))
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        moves
    }

    //if this function was instantaneous, we would save about 30% of our runtime :P
    //the mallocation isn't the issue, I think it's just that we iterate over ~6 cars 220,000 times!
    // fn add_to_moves(x: i32, y: i32, car: &Car, cars: &[Rc<Car>], g: usize) -> Board {
    //     let new_car = Rc::new(Car::new(car.vertical, car.length, car.colour, x, y));

    //     Board::from_cars(
    //         cars.iter()
    //             .map(|old_car| {
    //                 if old_car.colour == car.colour {
    //                     Rc::clone(&new_car)
    //                 } else {
    //                     Rc::clone(old_car)
    //                 }
    //             })
    //             .collect(),
    //         g,
    //     )
    // }

    pub fn is_solved(&self) -> bool {
        for car in &self.cars {
            if car.is_victorious() {
                return true;
            }
        }
        false
    }

    fn gen_heuristic(chars: [[char; 6]; 6]) -> usize {
        let mut retval = 1;
        for character in chars[2].iter().rev() {
            match character {
                'X' => break,
                '.' => continue,
                _ => retval += 1,
            }
        }
        retval
    }

    fn f(&self) -> usize {
        self.g + self.h
    }
}

fn str_to_chars(board_string: &str) -> [[char; 6]; 6] {
    let mut char_array = [['0'; 6]; 6];
    let mut seperated_board = board_string.lines();

    for y in 0..6 {
        let mut line: Vec<char> = seperated_board.next().unwrap().chars().collect();
        for x in 0..6 {
            char_array[y][5 - x] = line.pop().unwrap();
        }
    }

    char_array
}

// unsafe fn refclone(cars: *const Vec<Car) -> Vec<Car> {
//     let cars = *cars;

//     cars.clone()
// }

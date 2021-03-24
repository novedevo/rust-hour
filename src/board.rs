use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash};

// #[derive(PartialEq, Eq, Hash, Clone)]
// enum Size {
//     Car = 2,
//     Truck = 3,
// }

#[derive(PartialEq, Eq, Hash, Clone, Debug, Ord)] //Ord
struct Car {
    vertical: bool,
    length: i32,
    colour: char,
    x: i32,
    y: i32,
}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.colour.cmp(&other.colour))
    }
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

#[derive(Debug, Eq, Clone)]
pub struct Board {
    cars: HashSet<Car>,
    board_chars: [[char; 6]; 6],
    // heuristic: i32,
}

impl PartialEq for Board {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(0.cmp(&0))
    }
}

impl Ord for Board {
    fn cmp(&self, _other: &Self) -> Ordering {
        0.cmp(&0)
    }
}

// impl Hash for Board {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.ordered_cars().hash(state);
//     }
// }

impl Board {
    pub fn from_str(board_path: &str) -> Self {
        let board_string = match std::fs::read_to_string(board_path) {
            Ok(a) => {
                println!("{}", board_path);
                a
            }
            _ => panic!("{}", board_path),
        };
        let mut cars: HashSet<Car> = HashSet::new();
        let mut colours: HashSet<char> = HashSet::new();
        colours.insert('.');
        let chars = str_to_chars(&board_string);
        for (y, line) in board_string.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if !colours.contains(&tile) {
                    colours.insert(tile);
                    cars.insert(Car::new(
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
            // heuristic: 0,
        }
    }

    fn _blocking_heuristic(&self) -> i32 {
        let mut h = 0;
        for i in 5..1 {
            let tile = self.board_chars[2][i];
            if tile == 'X' {
                break;
            }
            else if tile == '.' {
                continue;
            }
            else {
                h += 1;
            }
        }
        h
    }
    
    fn from_cars(cars: HashSet<Car>) -> Self {
        Board {
            board_chars: Self::gen_chars(&cars),
            cars,
            // heuristic: 0,
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

    fn gen_chars(cars: &HashSet<Car>) -> [[char; 6]; 6] {
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

    fn _ordered_cars(&self) -> Vec<Car> {
        let cars = self.cars.clone();
        let mut retval = cars.into_iter().collect::<Vec<Car>>();
        retval.sort();
        retval
    }

    pub fn get_moves(&self) -> Vec<Self> {
        let mut moves: Vec<Self> = Vec::<Self>::with_capacity(10);

        for car in &self.cars {
            if !car.vertical {
                for i in 1..4 {
                    if car.x - i >= 0
                        && self.board_chars[car.y as usize][(car.x - i) as usize] == '.'
                    {
                        Self::add_to_moves(
                            car.x - i,
                            car.y,
                            car,
                            &self.cars,
                            &mut moves,
                            // self.heuristic + 1,
                        )
                    } else {
                        break;
                    }
                }
                for i in 1..4 {
                    if car.x + car.length + i < 6
                        && self.board_chars[car.y as usize][(car.x + car.length + i) as usize]
                            == '.'
                    {
                        Self::add_to_moves(
                            car.x + i,
                            car.y,
                            car,
                            &self.cars,
                            &mut moves,
                            // self.heuristic + 1,
                        )
                    } else {
                        break;
                    }
                }
            } else {
                for i in 1..4 {
                    if car.y - i >= 0
                        && self.board_chars[(car.y - i) as usize][car.x as usize] == '.'
                    {
                        Self::add_to_moves(
                            car.x,
                            car.y - i,
                            car,
                            &self.cars,
                            &mut moves,
                            // self.heuristic + 1,
                        )
                    } else {
                        break;
                    }
                }
                for i in 1..4 {
                    if car.y + car.length + i < 6
                        && self.board_chars[(car.y + car.length + i) as usize][car.x as usize]
                            == '.'
                    {
                        Self::add_to_moves(
                            car.x,
                            car.y + i,
                            car,
                            &self.cars,
                            &mut moves,
                            // self.heuristic + 1,
                        )
                    } else {
                        break;
                    }
                }
            }
        }

        moves
    }

    fn add_to_moves(
        x: i32,
        y: i32,
        car: &Car,
        cars: &HashSet<Car>,
        moves: &mut Vec<Board>,
        // heuristic: i32,
    ) {
        let new_car = Car::new(car.vertical, car.length, car.colour, x, y);
        let mut new_cars = HashSet::<Car>::with_capacity(cars.capacity());
        for old_car in cars {
            if old_car.colour == car.colour {
                new_cars.insert(new_car.clone());
            } else {
                new_cars.insert(old_car.clone());
            }
        }
        moves.push(Board::from_cars(new_cars));
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

fn str_to_chars(board_string: &str) -> [[char; 6]; 6] {
    // let characters: Vec<Vec<char>>;
    let mut char_array = [['0'; 6]; 6];
    let mut seperated_board = board_string.lines();

    use std::convert::TryInto;

    for y in 0..6 {
        let line: Vec<char> = seperated_board.next().unwrap().chars().collect();
        char_array[y] = line.try_into().unwrap()
    }
    // use std::convert::TryInto;

    // for (y, newline) in characters.enumerate() {
    //     char_array[y] = match newline.try_into() {
    //         Ok(arr) => arr,
    //         Err(_) => panic!("array conversion"),
    //     }
    // }

    char_array
}

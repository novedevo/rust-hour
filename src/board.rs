use std::collections::HashSet;

// #[derive(PartialEq, Eq, Hash, Clone)]
// enum Size {
//     Car = 2,
//     Truck = 3,
// }

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Car {
    vertical: bool,
    length: usize,
    colour: char,
    x: usize,
    y: usize,
}

impl Car {
    pub fn new(vertical: bool, length: usize, colour: char, x: usize, y: usize) -> Self {
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

#[derive(Debug)]
pub struct Board {
    cars: HashSet<Car>,
    board_chars: [[char; 6]; 6],
}

impl Board {
    pub fn from_str(board_path: &str) -> Self {
        let board_string = match std::fs::read_to_string(board_path) {
            Ok(a) => {
                println!("{}", board_path);
                a
            },
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
                        x,
                        y,
                    ));
                }
            }
        }
        Board {
            cars,
            board_chars: chars,
        }
    }
    fn from_cars(cars: HashSet<Car>) -> Self {
        Board {
            board_chars: Self::gen_chars(&cars),
            cars,
        }
    }
    fn gen_chars(cars: &HashSet<Car>) -> [[char; 6]; 6] {
        let mut retval = [['.'; 6]; 6];

        for car in cars {
            if car.vertical {
                for i in 0..car.length {
                    retval[car.y + i][car.x] = car.colour;
                }
            } else {
                for i in 0..car.length {
                    retval[car.y][car.x + i] = car.colour;
                }
            }
        }

        retval
    }
    fn is_vertical(board_string: [[char; 6]; 6], x: usize, y: usize) -> bool {
        y < 5 && board_string[y][x] == board_string[y + 1][x]
    }
    fn get_length(board_string: [[char; 6]; 6], x: usize, y: usize) -> usize {
        let colour = board_string[y][x];
        if Self::is_vertical(board_string, x, y) {
            if y < 4 && board_string[y + 2][x] == colour {
                3
            } else {
                2
            }
        } else {
            if x < 4 && board_string[y][x + 2] == colour {
                3
            } else {
                2
            }
        }
    }
    
    pub fn is_victorious(&self) -> bool {
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

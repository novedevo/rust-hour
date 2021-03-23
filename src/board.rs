use std::{collections::HashSet};

#[derive(PartialEq, Eq, Hash, Clone)]
enum Size {
    Car,
    Truck,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Car {
    vertical: bool,
    length: Size,
    colour: char,
}

impl Car {
    pub fn new(vertical: bool, length: Size, colour: char) -> Self {
        Car {
            vertical,
            length,
            colour,
        }
    }
}

pub struct Board {
    cars: HashSet<Car>,
    board_chars: [[char; 6]; 6],
}

impl Board {
    pub fn from_str(board_string: &str) -> Self {
        let board_string = std::fs::read_to_string(board_string).unwrap();
        let mut cars: HashSet<Car> = HashSet::new();
        let mut colours: HashSet<char> = HashSet::new();
        let chars = str_to_chars(&board_string);
        for (y, line) in board_string.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if !colours.contains(&tile) {
                    colours.insert(tile);
                    cars.insert(Car::new(
                        Self::is_vertical(chars, x, y),
                        Self::get_length(chars, x, y),
                        tile,
                    ));
                }
            }
        }
        Self::from_cars(&cars)
    }
    fn from_cars(cars: &HashSet<Car>) -> Self {
        Board {
            cars: cars.clone(),
            board_chars: Self::gen_chars(&cars),
        }
    }
    fn gen_chars(cars: &HashSet<Car>) -> [[char; 6]; 6] {
        [['0'; 6]; 6]
    }
    fn is_vertical(board_string: [[char; 6]; 6], x: usize, y: usize) -> bool {
        true
    }
    fn get_length(board_string: [[char; 6]; 6], x: usize, y: usize) -> Size {
        Size::Car
    }
}

fn str_to_chars(board_string: &str) -> [[char; 6]; 6] {
    // let characters: Vec<Vec<char>>;
    let mut char_array = [['0'; 6]; 6];
    let mut seperated_board = board_string.lines();

    use std::convert::TryInto;
    
    for y in 0..6 {
        let line:Vec<char> = seperated_board.next().unwrap().chars().collect();
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

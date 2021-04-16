use rustc_hash::FxHashSet; //extra-fast hashing algorithm
use std::{
    convert::TryInto, //to pull slice of bytes into 6-wide array of bytes when reading from string
    fmt::{Display, Formatter, Result, Write}, //to allow for outputting to file
};

//copyable because it is stored on the stack
#[derive(Clone, Copy)]
struct Car {
    vertical: bool,
    truck: bool,
    colour: u8,
    x: u8,
    y: u8,
}

// I wish we could derive Copy. Vectors are stored on the heap, and using arrays allocates way too much memory and slows it down anyway
// allocating and dropping all these vectors is the slowest part of the program.
#[derive(Clone)]
pub struct Board {
    cars: Vec<Car>,
    pub array: [[u8; 6]; 6],
    moves: Vec<[u8; 3]>,
}

//associated functions
impl Board {
    //allegedly a constructor
    //generate board from a path to a file
    //assumed to be a valid board. Providing an invalid board is UB. Panics when file is invalid
    pub fn from_path(board_path: &str) -> Self {
        let mut cars: Vec<Car> = Vec::with_capacity(15); //largest board in test suite has only 15 colours / cars
        let mut colours: FxHashSet<u8> = FxHashSet::default(); //so we reserve that amount
        colours.reserve(8); //actually reserves 16; rust likes to reserve double the amount you expect
        colours.insert(b'.'); // guard against the pavement
        let u8s = str_to_u8s(
            std::fs::read_to_string(board_path).expect("Error: could not read file. Panicking!"),
        ); //read the file, then convert it to a 6x6 byte array

        for (y, line) in u8s.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if !colours.contains(tile) {
                    //only procs when a new colour is presented
                    colours.insert(*tile);
                    cars.push(Car {
                        //add our new Car struct
                        vertical: Self::is_vertical(u8s, x, y),
                        truck: Self::is_truck(u8s, x, y),
                        colour: *tile,
                        x: x as u8,
                        y: y as u8,
                    });
                }
            }
        }

        Board {
            cars,
            array: u8s,
            moves: vec![],
        }
    }

    //hot path. Calculates the 6x6 byte array delimited by the slice of Cars
    fn gen_u8s(cars: &[Car]) -> [[u8; 6]; 6] {
        let mut retval = [[b'.'; 6]; 6];

        for car in cars {
            //contains only the finest artisanal hand-unrolled loops
            //we only need to handle four cases: vertical car, vertical truck, horizontal car, and horizontal truck
            if car.vertical {
                retval[(car.y) as usize][car.x as usize] = car.colour;
                retval[(car.y + 1) as usize][car.x as usize] = car.colour;
                if car.truck {
                    retval[(car.y + 2) as usize][car.x as usize] = car.colour;
                }
            } else {
                retval[car.y as usize][(car.x) as usize] = car.colour;
                retval[car.y as usize][(car.x + 1) as usize] = car.colour;
                if car.truck {
                    retval[car.y as usize][(car.x + 2) as usize] = car.colour;
                }
            }
        }

        retval
    }

    //fairly self-explanatory
    // tests one position to determine if the car is vertical or not
    // assumes correct board.
    const fn is_vertical(board_string: [[u8; 6]; 6], x: usize, y: usize) -> bool {
        y < 5 && board_string[y][x] == board_string[y + 1][x]
    }

    //cars can only be length 2 or 3. 3 is considered a truck.
    //this assumes a correct board.
    const fn is_truck(board_string: [[u8; 6]; 6], x: usize, y: usize) -> bool {
        let colour = board_string[y][x];
        if Self::is_vertical(board_string, x, y) {
            y < 4 && board_string[y + 2][x] == colour
        } else {
            x < 4 && board_string[y][x + 2] == colour
        }
    }

    //hot path. Calculates all adjacent board states.
    //this is only interior mutability. this could still be immutable externally, but that requires another allocation,
    //and considering how many times this function is called, I'd rather avoid that.
    pub fn get_moves(&mut self) -> impl Iterator<Item = Board> {
        let mut carses = Vec::with_capacity(20); //vec of vec of cars (and their associated movement trails), thus carses

        let cars = &mut self.cars as *mut Vec<Car>; //create raw mutable pointer

        // I think this is completely safe, actually. I never do anything too weird with memory.
        // the only reason I need unsafe is to replicate basically these lines of code:
        //
        // car.x += 1;
        // cars.clone();
        // car.x -= 1;
        //
        // however, this is still a 60 line unsafe block.
        //
        // this is not a place of honor
        // no great deed is commemorated here. nothing of value is here.
        // what is here is dangerous and repulsive to us
        // the danger is still present in your time as it was in ours
        unsafe {
            //dereference raw mutable pointer (unsafe)
            for car in &mut *cars {
                let length = if car.truck { 2 } else { 1 };

                if !car.vertical {
                    let mut i = 0; 
                    loop {
                        i += 1;
                        //because moving multiple steps is still a single move
                        if car.x >= i //check bounds
                            && self.array[car.y as usize][(car.x - i) as usize] == b'.'
                        //check that there is space
                        {
                            let turn = [car.colour, b'L', (i + 48).to_ascii_lowercase()]; // we want something of the format XR4 to signify car X, moving right, 4 units.
                                                                                          // thus we use a 3-wide byte array with the car's colour, then which direction we are going as a byte,
                                                                                          // then add 48 to our movement length to convert the integer into ascii digits

                            self.moves.push(turn); //this modifies self.moves

                            car.x -= i; //move the car left one space
                            carses.push(((*cars).clone(), self.moves.clone())); //copy the list of cars. this dereferences our pointer again, which is unsafe.
                                                                                //we now have a mutable reference and an immutable reference existing at the same time,
                                                                                //which is a recipe for disaster and rustc will refuse to compile it without using raw pointers like this
                                                                                //
                                                                                //this also clones the list of moves (including our most recent move), which is slightly less illegal
                                                                                //because we only have one reference at any given time

                            car.x += i; //replace the car to its original position
                            self.moves.pop(); //return self.moves to its original state
                        } else {
                            break; //to prevent phasing through thin cars
                        }
                    }
                    let mut i = 0; 
                    loop {
                        i += 1;
                        if car.x + length + i < 6
                            && self.array[car.y as usize][(car.x + length + i) as usize] == b'.'
                        {
                            let turn = [car.colour, b'R', (i + 48).to_ascii_lowercase()];
                            self.moves.push(turn);
                            car.x += i;
                            carses.push(((*cars).clone(), self.moves.clone()));
                            car.x -= i;
                            self.moves.pop();
                        } else {
                            break;
                        }
                    }
                } else {
                    //car is vertical
                    let mut i = 0; 
                    loop {
                        i += 1;
                        if car.y >= i && self.array[(car.y - i) as usize][car.x as usize] == b'.' {
                            let turn = [car.colour, b'U', (i + 48).to_ascii_lowercase()];
                            self.moves.push(turn);
                            car.y -= i;
                            carses.push(((*cars).clone(), self.moves.clone()));
                            car.y += i;
                            self.moves.pop();
                        } else {
                            break;
                        }
                    }
                    let mut i = 0; 
                    loop {
                        i += 1;
                        if car.y + length + i < 6
                            && self.array[(car.y + length + i) as usize][car.x as usize] == b'.'
                        {
                            let turn = [car.colour, b'D', (i + 48).to_ascii_lowercase()];
                            self.moves.push(turn);
                            car.y += i;
                            carses.push(((*cars).clone(), self.moves.clone()));
                            car.y -= i;
                            self.moves.pop();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        carses.into_iter().map(|(cars, turns)| Board {
            array: Self::gen_u8s(&cars),
            cars,
            moves: turns,
        }) //convert vec of vecs of cars and vecs of arrays of bytes into lazy iterator of boards
    }

    //fairly self-explanatory, I would imagine
    pub fn is_solved(&self) -> bool {
        self.array[2][5] == b'X'
    }
}

//to format the board into the files it should produce
//not the most efficient way to do it, but this only happens once per board and takes nanoseconds, so it's fine
impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for line in &self.moves {
            for c in line {
                f.write_char(*c as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

//converts a 6-line string of 6 characters each, properly formatted, into a 6x6 array of bytes
fn str_to_u8s(board_string: String) -> [[u8; 6]; 6] {
    let mut u8_array = [[b'0'; 6]; 6]; //initialize
    let mut seperated_board = board_string.lines();

    for row in &mut u8_array {
        *row = seperated_board
            .next() //get next line
            .expect("Invalid board: not enough lines")
            .as_bytes() //convert to ascii bytes //me want byte, me want ascii char delight
            .try_into() //attempt to convert from slice to 6-long array
            .expect("Invalid board");
    }

    u8_array
}

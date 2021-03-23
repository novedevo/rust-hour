mod board;
use std::{env, fs};

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        solo_test();
        multi_test();
    }

    fn solo_test() {
        let a = crate::board::Board::from_str("puzzles/A00.txt");
        println!("{:#?}", a)
    }

    fn multi_test() {
        for entry in crate::fs::read_dir("puzzles").unwrap() {
            let entry = entry.unwrap();
            let a = crate::board::Board::from_str(entry.path().to_str().unwrap());
            println!("{:?}", a)
        }
    }
}

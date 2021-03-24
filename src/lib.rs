mod board;
pub mod solver;


#[cfg(test)]
mod tests {
    use super::board::Board;
    use super::solver;
    
    #[test]
    fn main() {
        solo_test();
        multi_test();
    }

    fn solo_test() {
        let a = Board::from_str("puzzles/A00.txt");
        println!("{:#?}", a)
    }

    fn multi_test() {
        for entry in std::fs::read_dir("puzzles").unwrap() {
            let entry = entry.unwrap();
            let a = Board::from_str(entry.path().to_str().unwrap());
            assert!(solver::solve(a));
        }
    }
}

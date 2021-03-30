mod board;
pub mod solver;

#[cfg(test)]
mod tests {
    use super::board::Board;
    use super::solver;
    use std::thread;

    #[test]
    fn bench() {
        let mut threads = vec![];
        for entry in std::fs::read_dir("puzzles").unwrap() {
            let new_thread = thread::spawn(|| {
                let entry = entry.unwrap();
                let a = Board::from_str(entry.path().to_str().unwrap());
                solver::stress_solve(a);
            });
            threads.push(new_thread);
        }

        for handle in threads {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn solve() {
        solver::solve("puzzles/B19.txt", "solutions/B19.txt");
    }
}

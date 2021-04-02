mod board;
pub mod solver;

#[cfg(test)]
mod tests {
    use super::board::Board;
    use super::solver;

    #[test]
    fn bench() {
        let mut threads = vec![];
        for entry in std::fs::read_dir("puzzles").unwrap() {
            let new_thread = std::thread::spawn(|| {
                let entry = entry.unwrap();
                let a = Board::from_path(entry.path().to_str().unwrap());
                solver::stress_solve(a);
            });
            threads.push(new_thread);
        }

        for handle in threads {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn solo_solve() {
        std::fs::create_dir_all("solutions").unwrap();
        solver::solve("puzzles/B19.txt", "solutions/B19.txt");
    }
}

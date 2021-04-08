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
            threads.push(std::thread::spawn(|| {
                solver::stress_solve(Board::from_path(entry.unwrap().path().to_str().unwrap()));
            }));
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
    
    #[test]
    fn solve_all() {      
        std::fs::create_dir_all("solutions").unwrap();
        let mut threads = vec![];
        for entry in std::fs::read_dir("puzzles").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path().to_str().unwrap().to_owned();
            let name = entry.file_name().to_str().unwrap().to_owned();
            threads.push(std::thread::spawn(move || {
                solver::solve(&path, &format!("solutions/{}", name));
            }));
        }

        for handle in threads {
            handle.join().unwrap();
        }
    }
}

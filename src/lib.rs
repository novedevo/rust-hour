mod board;
pub mod solver;

#[cfg(test)]
mod tests {
    use super::board::Board;
    // use super::solver;
    // use std::{fs::File, io::Write, path::Path, thread};

    #[test]
    fn main() {
        println!("ADJAHSDGAJSHGD");
        let _board = Board::from_str("puzzles/A00.txt");
        // let _a = solver::solve(board);
        println!("{:#?}", 1)
        // solo_test();
        // multi_test();
    }

    // fn _solo_test() {
    //     let a = solver::solve(Board::from_str("puzzles/A00.txt"));
    //     println!("{:#?}", a)
    // }

    // fn _multi_test() {
    //     let mut threads = Vec::with_capacity(40);
    //     for entry in std::fs::read_dir("puzzles").unwrap() {
    //         let new_thread = thread::spawn(|| {
    //             let entry = entry.unwrap();
    //             let a = Board::from_str(entry.path().to_str().unwrap());
                
    //             let outfile_str = String::from("solutions/") + entry.file_name().to_str().unwrap();
                
    //             let out_path = Path::new(&outfile_str);
    //             let mut out_file = File::create(&out_path).unwrap();
                
                
    //             let a = solver::solve(a);
    //             out_file.write_all(a.to_str().iter().collect::<String>().as_bytes()).unwrap();
    //         });
    //         threads.push(new_thread);
    //     }
    //     for handle in threads {
    //         handle.join().unwrap();
    //     }
    // }
}

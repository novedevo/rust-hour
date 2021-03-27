use ahash::AHashSet; //10x faster than the default at the expense of some cryptographic defenses

use crate::board::Board;

//returns the solved board and a number indicating how many distinct states were checked
pub fn solve(board: Board) -> (Board, usize) { //wrapper
    dfs(board)
}

//dfs is ~30% less iterations than bfs in this situation, for some reason
fn dfs(board: Board) -> (Board, usize) {
    let mut visited: AHashSet<[[char; 6]; 6]> = AHashSet::new(); //keep track of all the nodes we have visited to avoid backtracking
    visited.insert(board.board_chars);

    let mut stack: Vec<Board> = Vec::new();
    stack.push(board);

    while let Some(board) = stack.pop() {
        for new_board in board.get_moves() {
            if new_board.is_solved() {
                return (new_board, visited.len());
            } else if !visited.contains(&new_board.board_chars) {
                visited.insert(new_board.board_chars);
                stack.push(new_board);
            }
        }
    }

    panic!("at the disco!");
}

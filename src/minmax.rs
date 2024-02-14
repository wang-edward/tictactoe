use board;
mod minmax {
    use crate::board::Board;
    use crate::board::Piece;
    struct Move {
        turn: board::Piece,
        x: usize,
        y: usize,
    }

    struct Evaluation {
        status: i32,
        best_move: Move
    }

    fn minmax(&board: Board, depth: i32) -> i32 {
        let board_copy = board.clone();    
        if depth == 0 || board.check_win() != Win::No {
            return evaluate(board)
        }
    }

    fn evaluate(&board: Board) -> i32 {
        let result = board.check_win();
        let winner = match (result) {
            Win::X => Piece::X,
            Win::O => Piece::O,
            Win::No => Piece::Empty,
            Win::No => Piece::Empty,
        };
        if (board.turn == winner) {
            return 2;
        } else if (result == Win::No) {
            return 0;
        } else if (result == Win::Tie) {
            return -1;
        } else {
            return -2;
        }
    }
}

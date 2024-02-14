use crate::board;
use crate::board::Win;
use crate::board::Piece;
use crate::board::Board;

#[derive(Debug)]
pub struct Move {
    turn: board::Piece,
    x: usize,
    y: usize,
}

pub fn negamax(board: &mut Board, depth: i32) -> (i32, Move) {
    let mut max_eval = -100;
    let mut best_move = Move{turn: Piece::Empty, x: usize::MAX, y: usize::MAX};
    if depth == 0 || board.check_win() != Win::No {
        return (evaluate(board), best_move);
    }

    for m in generate_moves(&board) {
        board.make_move(m.x, m.y);
        let (mut eval, _) = negamax(board, depth - 1);
        eval = -eval;
        board.undo_move(m.x, m.y);
        if eval > max_eval {
            max_eval = eval;
            best_move = m;
        }

    }

    return (max_eval, best_move);
}

fn evaluate(board: &board::Board) -> i32 {
    let result = board.check_win();
    let winner = match result {
        Win::X => Piece::X,
        Win::O => Piece::O,
        Win::No => Piece::Empty,
        Win::Tie => Piece::Empty,
    };
    if board.turn == winner {
        return 2;
    } else if result == Win::No {
        return 0;
    } else if result == Win::Tie {
        return -1;
    } else {
        return -2;
    }
}

fn generate_moves(board: &Board) -> Vec<Move> {
    let mut ans: Vec<Move> = Vec::new();
    for x in 0..board::SIZE {
        for y in 0..board::SIZE {
            if board.board[x][y] == Piece::Empty {
                ans.push(Move{turn: board.turn, x: x, y: y});
            }
        }
    }

    // println!("Possible moves: {:?}", ans);

    return ans;
}

const SIZE: usize = 3;

#[derive(Clone, Copy)]
enum Piece {X, O, Empty}

enum PlaceError {
    InvalidCoords,
    Occupied,
    Bad,
}

struct Board {
    board: [[Piece; SIZE]; SIZE],
    turn: Piece,
}

impl Board {
    fn make_move(&mut self, x: usize, y: usize) -> std::result::Result<(), PlaceError> {
        if x >= SIZE || y >= SIZE {
            return Err(PlaceError::InvalidCoords);
        }
        match self.board[x][y] {
            Piece::Empty => Ok(()),
            _ => Err(PlaceError::Occupied),
        }?;

        self.board[x][y] = self.turn;
        self.turn = match self.turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            Piece::Empty => return Err(PlaceError::Bad),
        };

        Ok(()) 
    }

    fn check_horizontal(self, row:usize) -> Piece {
        let mut x_win: bool = true;
        for i in 0..SIZE {
            x_win = match self.board[row][i] {
                Piece::O => false,
                _ => true,
            }
        }
    }

}

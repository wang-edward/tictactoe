use std::fmt;

const SIZE: usize = 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {X, O, Empty}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Piece::X => write!(f, "X"),
            Piece::O => write!(f, "O"),
            Piece::Empty => write!(f, "_"),
        }
    }
}

#[derive(Debug)]
pub enum PlaceError {
    InvalidCoords,
    Occupied,
    Bad,
}

pub struct Board {
    board: [[Piece; SIZE]; SIZE],
    turn: Piece,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board: [[Piece::Empty; SIZE]; SIZE],
            turn: Piece::X,
        }
    }
}

impl Board {
    pub fn make_move(&mut self, x: usize, y: usize) -> std::result::Result<(), PlaceError> {
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

    fn check_horizontal(&self, row: usize) -> Piece {
        let mut x_win = true;
        let mut o_win = true;
        for &cell in &self.board[row] {
            if cell != Piece::X {
                x_win = false;
            }
            if cell != Piece::O {
                o_win = false;
            }
        }
        if x_win {
            Piece::X
        } else if o_win {
            Piece::O
        } else {
            Piece::Empty
        }
    }

    fn check_vertical(&self, col:usize) -> Piece {
        let mut x_win = true;
        let mut o_win = true;
        for i in 0..SIZE {
            let &cell = &self.board[i][col];
            if cell != Piece::X {
                x_win = false;
            }
            if cell != Piece::O {
                o_win = false;
            }
        }
        if x_win {
            Piece::X
        } else if o_win {
            Piece::O
        } else {
            Piece::Empty
        }
    }

    fn check_diagonals(&self) -> Piece {
        let mut x_win_l = true;
        let mut x_win_r = true;
        let mut o_win_l = true;
        let mut o_win_r = true;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] != Piece::X {
                    x_win_l = false;
                }
                if self.board[i][j] != Piece::O {
                    o_win_l = false;
                }
                if self.board[(SIZE - 1) - i][(SIZE - 1) - j] != Piece::X {
                    x_win_r = false;
                }
                if self.board[(SIZE - 1) - i][(SIZE - 1) - j] != Piece::O {
                    o_win_r = false;
                }
            }
        }

        if x_win_l == true || x_win_r == true {
            return Piece::X
        } else if o_win_l == true || o_win_r == true {
            return Piece::O
        } else {
            return Piece::Empty
        }
    }

    pub fn check_win(&self) -> Piece {
        for i in 0..SIZE {
            match self.check_horizontal(i) {
                Piece::Empty => {},
                result => return result,
            }
            match self.check_vertical(i) {
                Piece::Empty => {},
                result => return result,
            }
        }
        return self.check_diagonals();
    }

    pub fn print(&self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                print!("{}", self.board[i][j]);
            }
            println!();
        }
    }
}

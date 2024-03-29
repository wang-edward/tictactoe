use std::fmt;

pub const SIZE: usize = 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {X, O, Empty}

#[derive(PartialEq, Eq)]
pub enum Win {X, O, No, Tie}

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
    NotOccupied,
    Bad,
}

pub struct Board {
    pub board: [[Piece; SIZE]; SIZE],
    pub turn: Piece,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board: [[Piece::Empty; SIZE]; SIZE],
            turn: Piece::X,
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut ans_board = [[Piece::Empty; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                ans_board[i][j] = self.board[i][j].clone();
            }
        }
        return Self{board: ans_board, turn: self.turn};
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

    pub fn undo_move(&mut self, x: usize, y: usize) -> std::result::Result<(), PlaceError> {
        if x >= SIZE || y >= SIZE {
            return Err(PlaceError::InvalidCoords);
        }

        match self.board[x][y] {
            Piece::Empty => Err(PlaceError::NotOccupied),
            _ => Ok(()),
        }?;

        self.board[x][y] = Piece::Empty;
        self.turn = match self.turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            Piece::Empty => return Err(PlaceError::Bad),
        };
        Ok(())
    }

    fn check_horizontal(&self, row: usize) -> Win {
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
            Win::X
        } else if o_win {
            Win::O
        } else {
            Win::No
        }
    }

    fn check_vertical(&self, col:usize) -> Win {
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
            Win::X
        } else if o_win {
            Win::O
        } else {
            Win::No
        }
    }

    fn check_diagonals(&self) -> Win {
        let mut x_win_l = true;
        let mut x_win_r = true;
        let mut o_win_l = true;
        let mut o_win_r = true;
        for i in 0..SIZE {
            if self.board[i][i] != Piece::X {
                x_win_l = false;
            }
            if self.board[i][i] != Piece::O {
                o_win_l = false;
            }
            if self.board[(SIZE - 1) - i][i] != Piece::X {
                x_win_r = false;
            }
            if self.board[(SIZE - 1) - i][i] != Piece::O {
                o_win_r = false;
            }
        }

        if x_win_l == true || x_win_r == true {
            return Win::X
        } else if o_win_l == true || o_win_r == true {
            return Win::O
        } else {
            return Win::No
        }
    }

    fn check_tie(&self) -> Win {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == Piece::Empty {
                    return Win::No;
                }
            }
        }
        return Win::Tie;
    }

    pub fn check_win(&self) -> Win {
        if self.check_tie() == Win::Tie {
            return Win::Tie;
        }
        for i in 0..SIZE {
            match self.check_horizontal(i) {
                Win::No => {},
                result => return result,
            }
            match self.check_vertical(i) {
                Win::No => {},
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

use std::io;
pub mod board;
mod negamax;

fn main() {
    let mut b: board::Board = board::Board::default();

    loop {
        b.print();
        println!("{:?}", negamax::negamax(&mut b, 10));
        match b.check_win() {
            board::Win::X => {
                println!("X is the winner!");
                break;
            },
            board::Win::O => {
                println!("O is the winner!");
                break;
            },
            board::Win::Tie => {
                println!("The game is a tie! How exciting!");
                break;
            }
            _ => {},
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut iter = input.split_whitespace();

        if input.eq_ignore_ascii_case("q") {
            break;
        }

        let result = match (iter.next(), iter.next()) {
            (Some(num1_str), Some(num2_str)) => {
                match (num1_str.parse::<usize>(), num2_str.parse::<usize>()) {
                    (Ok(num1), Ok(num2)) => b.make_move(num1, num2),
                    _ => {
                        println!("Error parsing");
                        Err(board::PlaceError::Bad)
                    }
                }
            }
            _ => {
                eprintln!("Invalid input format. Please enter two integers separated by a space.");
                Err(board::PlaceError::Bad)
            }
        };
        match result {
            Ok(()) => {},
            Err(err) => eprintln!("Move failed: {:?}", err),
        }
    }
}

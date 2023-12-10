mod minimax;
mod ttt;

use minimax::Minimax;
use std::io;
use std::io::Write;
use ttt::TicTacToe;

fn get_input() -> usize {
    loop {
        print!("Enter an index: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse() {
            Ok(parsed) => return parsed,
            Err(e) => {
                eprintln!("Error while getting input: {}", e);
                continue;
            }
        }
    }
}

fn main() {
    let mut ttt = TicTacToe::new();
    // change this line to play second or first
    let mm = Minimax::new(ttt::Player::O);

    ttt.print_board();

    loop {
        let playing = ttt.get_playing();
        println!("{}'s turn to play", playing);

        let input = if mm.get_playing() == playing {
            mm.get_move(&ttt)
        } else {
            get_input()
        };

        if let Err(e) = ttt.play_at(input) {
            println!("{}", e);
            continue;
        }

        if let Some(line) = ttt.check_win() {
            ttt.print_board_win(line);
            println!("Player {} wins!", ttt.get_other_player());
            break;
        }

        ttt.print_board();

        if !ttt.has_empty_squares() {
            println!("It's a tie!");
            break;
        }
    }
}

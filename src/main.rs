use ttt_rust::TicTacToe;
use std::io;
use std::io::Write;

fn get_input() -> usize {
    print!("Enter an index: ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input.trim().parse().expect("Input is not a valid integer")
}

fn main() {
    let mut ttt = TicTacToe::new();
    ttt.print_board();

    while ttt.has_empty_squares() {
        println!("{}'s turn to play", ttt.get_playing());
        let user_input = get_input();

        if let Err(e) = ttt.play_at(user_input) {
            println!("{}", e);
            continue;
        }

        if let Some(line) = ttt.check_win() {
            ttt.print_board_win(line);
            println!("Player {} wins!", ttt.get_other_player());
            break;
        }
        ttt.print_board();
    }
}

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

    for _ in 0..10 {
        println!("{}'s turn to play", ttt.get_playing());
        let user_input = get_input();

        if let Err(e) = ttt.play_at(user_input) {
            println!("{}", e);
            continue;
        }

        ttt.print_board();
    }
}

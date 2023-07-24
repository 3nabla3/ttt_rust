use ttt_rust::TicTacToe;

fn main() {
    let ttt = TicTacToe::new();
    ttt.print_board();
    println!("{}", ttt.get_playing());
}

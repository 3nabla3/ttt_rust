mod ttt_enums;
use ttt_enums::player2piece;
use ttt_enums::PlayerPiece;
use ttt_enums::Player;

use ansi_term::Color::RGB;

pub struct TicTacToe {
    board: [PlayerPiece; 9],
    playing: Player,
}

impl TicTacToe {
    const INDEX_COLOR: ansi_term::Color = RGB(100, 100, 100);

    pub fn new() -> TicTacToe {
        let initial_board = [PlayerPiece::NULL; 9];
        TicTacToe { board: initial_board, playing: Player::X }
    }

    pub fn get_playing(&self) -> Player { self.playing }

    fn print_row(&self, range: std::ops::Range<usize>) {
        for i in range {
            let c = if self.board[i] != PlayerPiece::NULL {
                self.board[i].to_string()
            } else {
                Self::INDEX_COLOR.paint(i.to_string()).to_string()
            };
            print!("| {} ", c);
        }
        println!("|");
    }

    pub fn print_board(&self) {
        println!(" --- --- --- ");
        self.print_row(0..3);
        println!(" --- --- --- ");
        self.print_row(3..6);
        println!(" --- --- --- ");
        self.print_row(6..9);
        println!(" --- --- --- ");
    }

    fn flip_playing(&mut self) {
        self.playing = match self.playing {
            Player::X => Player::O,
            Player::O => Player::X
        };
    }

    pub fn play_at(&mut self, index: usize) -> Result<(), &str> {
        if index >= 9 {
            return Err("The index should be between 0 and 8 inclusive");
        }

        match self.board[index] {
            PlayerPiece::NULL => self.board[index] = player2piece(&self.playing),
            _ => return Err("There is already a piece at that index")
        }
        
        self.flip_playing();

        Ok(())
    }

}

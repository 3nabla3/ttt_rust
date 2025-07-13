mod ttt_enums;

pub use ttt_enums::player2piece;
pub use ttt_enums::Player;
pub use ttt_enums::PlayerPiece;

use ansi_term::Color::RGB;

#[derive(Debug)]
pub struct TicTacToe {
    board: [PlayerPiece; 9],
    playing: Player,
}

impl TicTacToe {
    const INDEX_COLOR: ansi_term::Color = RGB(100, 100, 100);
    const WIN_COLOR: ansi_term::Color = RGB(255, 0, 0);
    const LINES_TO_CHECK: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8], // rows
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8], // columns
        [0, 4, 8],
        [6, 4, 2], // diags
    ];

    pub fn new() -> TicTacToe {
        let initial_board = [PlayerPiece::Empty; 9];
        TicTacToe {
            board: initial_board,
            playing: Player::X,
        }
    }

    pub fn new_with(board: [PlayerPiece; 9], playing: Player) -> TicTacToe {
        TicTacToe { board, playing }
    }

    pub fn get_playing(&self) -> Player {
        self.playing
    }

    pub fn get_board(&self) -> &[PlayerPiece; 9] {
        &self.board
    }

    fn get_char_display(&self, i: usize, line: Option<&[usize; 3]>) -> String {
        if let Some(line) = line {
            if line.contains(&i) {
                return Self::WIN_COLOR.paint(self.board[i].to_string()).to_string();
            }
        }

        if self.board[i] == PlayerPiece::Empty {
            return Self::INDEX_COLOR.paint(i.to_string()).to_string();
        }

        self.board[i].to_string()
    }

    fn print_row(&self, range: std::ops::Range<usize>, line: Option<&[usize; 3]>) {
        for i in range {
            let c = self.get_char_display(i, line);
            print!("| {c} ");
        }
        println!("|");
    }

    pub fn print_board(&self) {
        println!(" --- --- --- ");
        self.print_row(0..3, None);
        println!(" --- --- --- ");
        self.print_row(3..6, None);
        println!(" --- --- --- ");
        self.print_row(6..9, None);
        println!(" --- --- --- ");
    }

    pub fn print_board_win(&self, line: &[usize; 3]) {
        println!(" --- --- --- ");
        self.print_row(0..3, Some(line));
        println!(" --- --- --- ");
        self.print_row(3..6, Some(line));
        println!(" --- --- --- ");
        self.print_row(6..9, Some(line));
        println!(" --- --- --- ");
    }

    pub fn get_other_player(&self) -> Player {
        match self.playing {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn flip_playing(&mut self) {
        self.playing = self.get_other_player();
    }

    pub fn play_at(&mut self, index: usize) -> Result<(), &str> {
        if index >= 9 {
            return Err("The index should be between 0 and 8 inclusive");
        }

        match self.board[index] {
            PlayerPiece::Empty => self.board[index] = player2piece(&self.playing),
            _ => return Err("There is already a piece at that index"),
        }

        self.flip_playing();

        Ok(())
    }

    pub fn check_win(&self) -> Option<&[usize; 3]> {
        // we only have to check if the player that played last
        // made a 3 in a row
        let other_piece = player2piece(&self.get_other_player());

        Self::LINES_TO_CHECK.iter().find(|line| {
            self.board[line[0]] == other_piece
                && self.board[line[1]] == other_piece
                && self.board[line[2]] == other_piece
        })
    }

    pub fn has_empty_squares(&self) -> bool {
        self.board.contains(&PlayerPiece::Empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playing() {
        let mut ttt = TicTacToe::new();

        assert_eq!(ttt.get_playing(), Player::X);
        assert!(ttt.play_at(0).is_ok());

        assert_eq!(ttt.get_playing(), Player::O);
        assert!(ttt.play_at(1).is_ok());

        assert_eq!(ttt.get_playing(), Player::X);
        assert!(ttt.play_at(0).is_err());
        assert_eq!(ttt.get_playing(), Player::X);

        assert!(ttt.play_at(8).is_ok());
        assert_eq!(ttt.get_playing(), Player::O);

        assert!(ttt.play_at(9).is_err());
        assert_eq!(ttt.get_playing(), Player::O);
    }

    #[test]
    fn test_board() {
        let mut ttt = TicTacToe::new();

        assert_eq!(
            ttt.get_board(),
            &[
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
            ]
        );

        assert!(ttt.play_at(0).is_ok());
        assert!(ttt.play_at(1).is_ok());
        assert!(ttt.play_at(2).is_ok());
        assert!(ttt.play_at(3).is_ok());
        assert!(ttt.play_at(7).is_ok());
        assert!(ttt.play_at(8).is_ok());

        assert_eq!(
            ttt.get_board(),
            &[
                PlayerPiece::X,
                PlayerPiece::O,
                PlayerPiece::X,
                PlayerPiece::O,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::O,
            ]
        );
    }
}

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum PlayerPiece {
    NULL, X, O 
}

impl fmt::Display for PlayerPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerPiece::X => write!(f, "X"),
            PlayerPiece::O => write!(f, "O"),
            PlayerPiece::NULL => write!(f, "-"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Player {
    X, O,
}

pub fn player2piece(player: &Player) -> PlayerPiece {
    match player {
        Player::X => PlayerPiece::X,
        Player::O => PlayerPiece::O,
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        player2piece(&self).fmt(f)
    }
}

pub struct TicTacToe {
    board: [PlayerPiece; 9],
    playing: Player,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        let initial_board = [PlayerPiece::NULL; 9];
        TicTacToe { board: initial_board, playing: Player::X }
    }

    pub fn get_playing(&self) -> Player { self.playing }

    pub fn print_board(&self) {
        for (i, piece) in self.board.iter().enumerate() {
            print!("{}", piece);
            if i % 3 == 2 {
                println!();
            }
        }
    }
}

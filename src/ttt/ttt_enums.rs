use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

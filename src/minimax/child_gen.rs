use crate::ttt::player2piece;
use crate::ttt::PlayerPiece;
use crate::TicTacToe;

pub struct ChildGen<'a> {
    ttt: &'a TicTacToe,
    next_idx_to_check: usize,
}

pub fn generate_children(ttt: &TicTacToe) -> ChildGen {
    ChildGen {
        ttt,
        next_idx_to_check: 0,
    }
}

impl Iterator for ChildGen<'_> {
    type Item = (usize, TicTacToe);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.next_idx_to_check..9 {
            if self.ttt.get_board()[i] == PlayerPiece::Empty {
                // The deref will copy the board
                let mut board = *self.ttt.get_board();
                board[i] = player2piece(&self.ttt.get_playing());

                let child = TicTacToe::new_with(board, self.ttt.get_other_player());
                self.next_idx_to_check = i + 1;
                return Some((i, child));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ttt::Player;

    #[test]
    fn test_child_gen_count() {
        let ttt = TicTacToe::new();
        let children = generate_children(&ttt);

        assert_eq!(children.count(), 9);
    }

    #[test]
    fn test_child_gen_items() {
        let ttt = TicTacToe::new();
        let mut children = generate_children(&ttt);

        let (delta, ttt) = children.next().unwrap();
        assert_eq!(delta, 0);
        assert_eq!(
            *ttt.get_board(),
            [
                PlayerPiece::X,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty
            ]
        );

        let (delta, ttt) = children.next().unwrap();
        assert_eq!(delta, 1);
        assert_eq!(
            *ttt.get_board(),
            [
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty
            ]
        );

        let (delta, ttt) = children.next().unwrap();
        assert_eq!(delta, 2);
        assert_eq!(
            *ttt.get_board(),
            [
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty,
                PlayerPiece::Empty
            ]
        );
    }

    #[test]
    fn test_child_gen_items2() {
        let ttt = TicTacToe::new_with(
            [
                PlayerPiece::X,
                PlayerPiece::O,
                PlayerPiece::O,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::X,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::O,
            ],
            Player::O,
        );
        let mut children = generate_children(&ttt);
        let (delta, ttt) = children.next().unwrap();

        assert_eq!(delta, 3);
        assert_eq!(
            *ttt.get_board(),
            [
                PlayerPiece::X,
                PlayerPiece::O,
                PlayerPiece::O,
                PlayerPiece::O,
                PlayerPiece::X,
                PlayerPiece::X,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::O
            ],
        );

        let (delta, ttt) = children.next().unwrap();

        assert_eq!(delta, 6);
        assert_eq!(
            *ttt.get_board(),
            [
                PlayerPiece::X,
                PlayerPiece::O,
                PlayerPiece::O,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::X,
                PlayerPiece::O,
                PlayerPiece::X,
                PlayerPiece::O
            ],
        );
    }
}

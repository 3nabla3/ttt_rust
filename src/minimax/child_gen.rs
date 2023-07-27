use crate::TicTacToe;
use crate::ttt::PlayerPiece;
use crate::ttt::player2piece;

pub struct ChildGen<'a> {
    ttt: &'a TicTacToe,
    next_idx_to_check: usize,
}

pub fn generate_children(ttt: &TicTacToe) -> ChildGen {
    ChildGen { ttt, next_idx_to_check: 0 }
}

impl Iterator for ChildGen<'_> {
    type Item = (usize, TicTacToe);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.next_idx_to_check..9 {

            if self.ttt.get_board()[i] == PlayerPiece::NULL {
                let mut board = self.ttt.get_board().clone();
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
        assert_eq!(*ttt.get_board(), 
            [
                PlayerPiece::X, PlayerPiece::NULL, PlayerPiece::NULL,
                PlayerPiece::NULL,PlayerPiece::NULL,PlayerPiece::NULL,
                PlayerPiece::NULL,PlayerPiece::NULL,PlayerPiece::NULL
            ]
        );

        let (delta, ttt) = children.next().unwrap();
        assert_eq!(delta, 1);
        assert_eq!(*ttt.get_board(), 
            [
                PlayerPiece::NULL, PlayerPiece::X, PlayerPiece::NULL,
                PlayerPiece::NULL,PlayerPiece::NULL,PlayerPiece::NULL,
                PlayerPiece::NULL,PlayerPiece::NULL,PlayerPiece::NULL
            ]
        );

        let (delta, ttt) = children.next().unwrap();
        assert_eq!(delta, 2);
        assert_eq!(*ttt.get_board(), 
            [
                PlayerPiece::NULL, PlayerPiece::NULL, PlayerPiece::X,
                PlayerPiece::NULL,PlayerPiece::NULL,PlayerPiece::NULL,
                PlayerPiece::NULL,PlayerPiece::NULL,PlayerPiece::NULL
            ]
        );

    }
}

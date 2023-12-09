use std::cmp;

use crate::ttt::Player;
use crate::ttt::TicTacToe;

mod child_gen;

use child_gen::generate_children;
use rand::seq::SliceRandom;

pub struct Node {
    ttt: TicTacToe,
    pub children: Vec<Node>,  // Vector elements are already on the heap
}

impl Node {
    pub fn new(ttt: TicTacToe) -> Node {
        let children: Vec<Node> = Vec::new();
        Node { ttt, children }
    }
}

pub struct Minimax {
    playing: Player,
}

impl Minimax {
    const DAMP_FACTOR: f32 = 0.99;

    pub fn new(playing: Player) -> Minimax {
        Minimax { playing }
    }

    pub fn get_playing(&self) -> Player {
        self.playing
    }

    pub fn get_move(&self, ttt: &TicTacToe) -> usize {
        let mut best_score: Option<i32> = None;
        let mut best_indices: Vec<usize> = Vec::new();

        let maximizing = ttt.get_playing() == Player::X;

        for (delta, child) in generate_children(ttt) {
            let mut node = Node::new(child);
            let score = Self::get_node_score(&mut node, !maximizing);

            match best_score {
                Some(bs) => {
                    if maximizing && score > bs || !maximizing && score < bs {
                        best_indices.clear();
                        best_indices.push(delta);
                        best_score = Some(score);
                    } else if score == bs {
                        best_indices.push(delta);
                    }
                }
                None => {
                    best_indices.push(delta);
                    best_score = Some(score);
                }
            }
        }

        *best_indices
            .choose(&mut rand::thread_rng())
            .expect("Could not find a best move")
    }

    fn get_node_score(node: &mut Node, maximizing: bool) -> i32 {
        Self::minimax(node, i32::MIN, i32::MAX, maximizing)
    }

    fn static_analysis(ttt: &TicTacToe) -> i32 {
        match ttt.get_other_player() {
            Player::X => i32::MAX,
            Player::O => i32::MIN,
        }
    }

    fn minimax(node: &mut Node, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
        if node.ttt.check_win().is_some() {
            return Self::static_analysis(&node.ttt);
        } else if !node.ttt.has_empty_squares() {
            return 0;
        }

        let mut best_val = if maximizing {
            i32::MIN
        } else {
            i32::MAX
        };

        for (_delta, child) in generate_children(&node.ttt) {
            let mut child_node = Node::new(child);
            let val = Self::minimax(&mut child_node, alpha, beta, !maximizing);

            if maximizing {
                best_val = cmp::max(best_val, val);
                alpha = cmp::max(alpha, best_val);
                if best_val > beta {
                    break;
                }
            } else {
                best_val = cmp::min(best_val, val);
                beta = cmp::min(beta, best_val);
                if best_val < alpha {
                    break;
                }
            }
        }

        (best_val as f32 * Self::DAMP_FACTOR) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ttt::*;

    fn mm_get_move(ttt: &TicTacToe) -> usize {
        let mm = Minimax::new(ttt.get_playing());
        mm.get_move(ttt)
    }

    #[test]
    fn test_prevent_immediate_loss_row() {
        use PlayerPiece::*;
        let ttt = TicTacToe::new_with(
            [X, X, Empty, Empty, Empty, Empty, Empty, O, Empty],
            Player::O,
        );

        assert_eq!(mm_get_move(&ttt), 2);
    }

    #[test]
    fn test_prevent_immediate_loss_col() {
        use PlayerPiece::*;
        let ttt = TicTacToe::new_with(
            [O, X, Empty, Empty, X, Empty, Empty, Empty, Empty],
            Player::O,
        );

        assert_eq!(mm_get_move(&ttt), 7);
    }

    #[test]
    fn test_prevent_immediate_loss_diag() {
        use PlayerPiece::*;
        let ttt = TicTacToe::new_with(
            [X, Empty, Empty, Empty, X, O, Empty, Empty, Empty],
            Player::O,
        );

        assert_eq!(mm_get_move(&ttt), 8);
    }

    #[test]
    fn test_win_immediate_row() {
        use PlayerPiece::*;
        let ttt = TicTacToe::new_with(
            [Empty, X, Empty, Empty, X, X, O, O, Empty],
            Player::O,
        );

        assert_eq!(mm_get_move(&ttt), 8);
    }

    #[test]
    fn test_win_immediate_col() {
        use PlayerPiece::*;
        let ttt = TicTacToe::new_with(
            [Empty, Empty, Empty, Empty, O, X, X, O, X],
            Player::O,
        );

        assert_eq!(mm_get_move(&ttt), 1);
    }

    #[test]
    fn test_win_immediate_diag() {
        use PlayerPiece::*;
        let ttt = TicTacToe::new_with(
            [Empty, Empty, Empty, Empty, O, X, X, X, O],
            Player::O,
        );

        assert_eq!(mm_get_move(&ttt), 0);
    }

    fn play_2_mm(mm_x: &Minimax, mm_o: &Minimax) {
        let mut ttt = TicTacToe::new();

        while ttt.has_empty_squares() {
            let input = if ttt.get_playing() == Player::X {
                mm_x.get_move(&ttt)
            } else {
                mm_o.get_move(&ttt)
            };

            let result = ttt.play_at(input);
            // the algo should always play a legal move
            assert!(result.is_ok());

            let win = ttt.check_win();
            // the algo should never be beat
            assert!(win.is_none());
        }
    }

    #[test]
    fn test_always_tie_against_self() {
        let num_plays = 5;

        let mm_x = Minimax::new(Player::X);
        let mm_o = Minimax::new(Player::O);

        for _ in 0..num_plays {
            play_2_mm(&mm_x, &mm_o);
        }
    }
}

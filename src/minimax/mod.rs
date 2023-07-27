use std::cmp;

use crate::ttt::Player;
use crate::ttt::TicTacToe;
mod child_gen;
use child_gen::generate_children;
use rand::seq::SliceRandom;

pub struct Node {
    ttt: TicTacToe,

    pub children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(ttt: TicTacToe) -> Node {
        let children: Vec<Box<Node>> = Vec::new();
        Node { ttt, children }
    }
}

pub struct Minimax {
    playing: Player,
}

impl Minimax {
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
        Self::minimax(node, std::i32::MIN, std::i32::MAX, maximizing)
    }

    fn static_analysis(ttt: &TicTacToe) -> i32 {
        match ttt.get_other_player() {
            Player::X => std::i32::MAX,
            Player::O => std::i32::MIN,
        }
    }

    fn minimax(node: &mut Node, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
        if node.ttt.check_win().is_some() {
            return Self::static_analysis(&node.ttt);
        } else if !node.ttt.has_empty_squares() {
            return 0;
        }

        let mut best_val = if maximizing {
            std::i32::MIN
        } else {
            std::i32::MAX
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

        best_val
    }
}

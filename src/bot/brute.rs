use std::{collections::HashSet, isize};

use infinitable::Infinitable;
use rand::{seq::SliceRandom, thread_rng};

use crate::{board::Board, suit::Suit, vec2::Vec2};

use super::Bot;

pub struct Brute {
    win_len: usize,
    depth: usize,
}

impl Brute {
    pub fn new(win_len: usize, depth: usize) -> Self {
        Self {
            win_len,
            depth,
        }
    }
}

impl Bot for Brute {
    fn play(&mut self, board: &Board, last: Option<Vec2>) -> Vec2 {
        let mut board = board.clone();

        let (_, _, positions) = get_score(&mut board, self.depth);

        positions[..].choose(&mut thread_rng()).copied().unwrap_or_else(|| board.size().saturating_sub((1, 1)) / 2)
    }
}

fn get_score(board: &mut Board, depth: usize) -> (Suit, isize, Vec<Vec2>) {
    let me = board.on_turn();

    match board.check_win() {
        Some(s) if s == me => return (s, 1, vec![]),
        Some(Suit::None) => return (Suit::None, 0, vec![]),
        Some(s) => return (s, -1, vec![]),
        _ => {}
    }

    if depth == 0 {
        return (Suit::None, 0, vec![]);
    }

    let mut viable = HashSet::new();
    for pos in Vec2::new(0, 0).to(board.size()) {
        if !board[pos].is_none() {
            continue;
        }

        if pos.surround().any(|p| p.lt_and(board.size()) && !board[p].is_none()) {
            viable.insert(pos);
            viable.extend(pos.surround().filter(|p| p.lt_and(board.size()) && board[*p].is_none()));
        }
    }

    if viable.is_empty() {
        return (Suit::None, 0, vec![]);
    }

    let mut total_wins = 0;
    let mut best_score = me.oposite();
    let mut best_wins = isize::MIN;
    let mut positions = vec![];

    for pos in viable {
        board.set_selected(pos);
        _ = board.play();

        let (score, w, _) = get_score(board, depth - 1);
        let w = -w;
        total_wins += w;
        board[pos] = Suit::None;
        board.swap_turn();

        if score == me {
            if best_score == me {
                if w > best_wins {
                    best_wins = w;
                    positions.clear();
                    positions.push(pos);
                } else if w == best_wins {
                    positions.push(pos);
                }
            } else {
                best_score = me;
                best_wins = w;
                positions.clear();
                positions.push(pos);
            }
        } else if score == Suit::None {
            if best_score == me {
                continue;
            }
            if best_score == Suit::None {
                if w > best_wins {
                    best_wins = w;
                    positions.clear();
                    positions.push(pos);
                } else if w == best_wins {
                    positions.push(pos);
                }
            } else {
                best_score = Suit::None;
                best_wins = w;
                positions.clear();
                positions.push(pos);
            }
        } else {
            if best_score == me.oposite() {
                if w > best_wins {
                    best_wins = w;
                    positions.clear();
                    positions.push(pos);
                } else if w == best_wins {
                    positions.push(pos);
                }
            }
        }

        if score == me {
            break;
        }
    }

    return (best_score, total_wins, positions);
}

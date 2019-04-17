#![feature(generators, generator_trait)]
extern crate wasm_bindgen;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Move {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

#[wasm_bindgen]
extern "C" {
    fn alert(x: &str);
}

const ROW: u64 = 0xFFu64;
const COL: u64 = 0x0101010101010101u64;
const SLASH0: u64 = 0x0102040810204080u64;
const SLASH1: u64 = 0x8040201008040201u64;

trait Board {
    fn from_sparse_board(sparse: &[i32]) -> Self;
}

impl Board for [u64; 2] {
    fn from_sparse_board(sparse: &[i32]) -> Self {
        let mut board = [0u64; 2];
        for i in 0..8 {
            for j in 0..8 {
                let id = sparse[j + i * 8];
                if id >= 0 {
                    board[id as usize] |= 1u64 << (j + 8 * i);
                }
            }
        }
        board
    }
}

trait Coord {
    fn to_coord_2d(self) -> (i32, i32);
    fn to_piece(self) -> u64;
    fn higher_eq(self) -> u64;
    fn lower_eq(self) -> u64;
}

impl Coord for i32 {
    fn to_coord_2d(self) -> (i32, i32) {
        (self & 0x7, self >> 3)
    }
    fn to_piece(self) -> u64 {
        1u64 << self
    }
    fn higher_eq(self) -> u64 {
        -(self.to_piece() as i64) as u64
    }
    fn lower_eq(self) -> u64 {
        !self.higher_eq() | self.to_piece()
    }
}

trait Coord2D {
    fn to_coord(self) -> i32;
}

impl Coord2D for (i32, i32) {
    fn to_coord(self) -> i32 {
        self.0 | (self.1 << 3)
    }
}

fn gen_moves<'a>(
    board: &'a [u64; 2],
    turn: i32,
    pos: i32,
) -> impl Generator<Yield = i32, Return = ()> + 'a {
    move || {

        if (board[turn as usize] & pos.to_piece()) == 0 {
            return;
        }

        let check = move |dst: i32| (board[turn as usize] & dst.to_piece()) == 0;

        let (x, y) = pos.to_coord_2d();
        let tot = board[0] | board[1];
        let oppo = board[(1 - turn) as usize];

        {
            let col = COL << x;
            let col_cnt = (tot & col).count_ones() as i32;

            let top = pos + (col_cnt << 3);
            let bottom = pos - (col_cnt << 3);
            if top < 64 && check(top) {
                if (!(top.higher_eq() | pos.lower_eq()) & col & oppo) == 0 {
                    yield top;
                }
            }
            if bottom >= 0 && check(bottom) {
                if (!(pos.higher_eq() | bottom.lower_eq()) & col & oppo) == 0 {
                    yield bottom;
                }
            }
        }
        {
            let row = ROW << (y << 3);
            let row_cnt = (tot & row).count_ones() as i32;

            let left = pos + row_cnt;
            let right = pos - row_cnt;
            if x + row_cnt < 8 && check(left) {
                if (!(left.higher_eq() | pos.lower_eq()) & row & oppo) == 0 {
                    yield left;
                }
            }
            if x - row_cnt >= 0 && check(right) {
                if (!(pos.higher_eq() | right.lower_eq()) & row & oppo) == 0 {
                    yield right;
                }
            }
        }
        {
            let pl = x + y;
            let slash = if pl < 7 {
                SLASH0 >> ((7 - pl) << 3)
            } else {
                SLASH0 << ((pl - 7) << 3)
            };
            let slash_cnt = (tot & slash).count_ones() as i32;

            let top_right = pos + (7 * slash_cnt);
            let bottom_left = pos - (7 * slash_cnt);
            if x - slash_cnt >= 0 && y + slash_cnt < 8 && check(top_right) {
                if (!(top_right.higher_eq() | pos.lower_eq()) & slash & oppo) == 0 {
                    yield top_right;
                }
            }
            if x + slash_cnt < 8 && y - slash_cnt >= 0 && check(bottom_left) {
                if (!(pos.higher_eq() | bottom_left.lower_eq()) & slash & oppo) == 0 {
                    yield bottom_left;
                }
            }
        }
        {
            let pl = x - y;
            let slash = if pl > 0 {
                SLASH1 >> (pl << 3)
            } else {
                SLASH1 << (-pl << 3)
            };
            let slash_cnt = (tot & slash).count_ones() as i32;

            let top_left = pos + (9 * slash_cnt);
            let bottom_right = pos - (9 * slash_cnt);
            if x + slash_cnt < 8 && y + slash_cnt < 8 && check(top_left) {
                if (!(top_left.higher_eq() | pos.lower_eq()) & slash & oppo) == 0 {
                    yield top_left;
                }
            }
            if x - slash_cnt >= 0 && y - slash_cnt >= 0 && check(bottom_right) {
                if (!(pos.higher_eq() | bottom_right.lower_eq()) & slash & oppo) == 0 {
                    yield bottom_right;
                }
            }
        }
    }
}

fn gen_all_moves<'a>(
    board: &'a [u64; 2],
    turn: i32,
) -> impl Generator<Yield = (i32, i32), Return = ()> + 'a {
    move || {
        let tot = board[turn as usize];
        for pos in 0..64 {
            if (tot & pos.to_piece()) != 0 {
                let mut moves = gen_moves(&board, turn, pos);
                loop {
                    match Pin::new(&mut moves).resume() {
                        GeneratorState::Yielded(dst) => yield (pos, dst),
                        _ => break,
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn my_plain_solution(turn: i32, sparse: &[i32]) -> Move {
    let board = <[u64; 2]>::from_sparse_board(sparse);

    // alert(&format!("{:X}", board[0]));
    // alert(&format!("{:X}", board[1]));

    let w: Vec<(i32, i32)> = (0..64)
        .filter(|x| (board[0] & x.to_piece()) != 0)
        .map(|x| x.to_coord_2d())
        .collect();

    // alert(&format!("{:?}", w));

    let mut all_moves_vec = vec![];
    let mut all_moves = gen_all_moves(&board, turn);

    loop {
        match Pin::new(&mut all_moves).resume() {
            GeneratorState::Yielded(x) => all_moves_vec.push(x),
            _ => break,
        }
    }

    let all_moves_vec_2d: Vec<Move> = all_moves_vec
        .iter()
        .map(|x| {
            let (x0, y0) = x.0.to_coord_2d();
            let (x1, y1) = x.1.to_coord_2d();
            Move { x0, y0, x1, y1 }
        })
        .collect();

    // let u: Vec<(i32, i32)> = v.iter().map(|x| x.to_coord_2d()).collect();

    // alert(&format!("{:?}", all_moves_vec.iter().map(|x| {
    //     let (x0, y0) = x.0.to_coord_2d();
    //     let (x1, y1) = x.1.to_coord_2d();
    //     Move{x0, y0, x1, y1}
    // }).collect::<Vec<Move>>()));

    all_moves_vec_2d[0].clone()
}

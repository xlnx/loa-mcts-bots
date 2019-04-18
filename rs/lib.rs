#![feature(generators, generator_trait)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate wbg_rand;
use wbg_rand::{wasm_rng, Rng, WasmRng};

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

struct FakeRng {
    curr: i32,
    list: Vec<f32>,
}

impl FakeRng {
    fn gen(&mut self) -> f32 {
        self.curr += 1;
        let id = self.curr % self.list.len() as i32;
        self.list[id as usize]
    }
    fn new(n: i32) -> Self {
        let mut list: Vec<f32> = vec![];
        for _i in 0..n {
            list.push(0.5f32);
        }
        FakeRng { curr: 0, list }
    }
}

type RngProvider = WasmRng;

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
    fn log(x: &str);
}

const ROW: u64 = 0xFFu64;
const COL: u64 = 0x0101010101010101u64;
const SLASH0: u64 = 0x0102040810204080u64;
const SLASH1: u64 = 0x8040201008040201u64;
const C: f32 = 1.414f32 * 3e-1;

// const MAX_STEP: usize = 512;
// const MAX_NODE: usize = 128;
const MAX_STEP: usize = 256;
const MAX_NODE: usize = 512;

const EMPTY_MOVE: (i32, i32) = (100, 100);

struct DisjointSet {
    pre: [i32; 64],
}

impl DisjointSet {
    fn join(&mut self, mut a: i32, mut b: i32) {
        a = self.get(a);
        b = self.get(b);
        if a != b {
            self.pre[b as usize] = a;
        }
    }
    fn get(&mut self, mut a: i32) -> i32 {
        while self.pre[a as usize] != -1 {
            a = self.pre[a as usize]
        }
        a
    }
    fn is_joint(&mut self, v: &Vec<i32>) -> bool {
        let u: Vec<i32> = v.iter().map(|x| self.get(*x)).collect();
        u.iter().all(|x| *x == u[0])
    }
}

trait Board {
    fn from_sparse_board(sparse: &[i32], turn: i32) -> Self;
    fn apply_move(&self, src: i32, dst: i32) -> Self;
    fn is_win_state(&self) -> Option<bool>;
    fn gen_rand_move(&self, turn: i32, rng: &mut RngProvider) -> (i32, i32);
}

impl Board for [u64; 2] {
    fn from_sparse_board(sparse: &[i32], turn: i32) -> Self {
        let mut board = [0u64; 2];
        for i in 0..8 {
            for j in 0..8 {
                let id = sparse[j + i * 8];
                if id >= 0 {
                    board[id as usize] |= 1u64.overflowing_shl((j + 8 * i) as u32).0;
                }
            }
        }
        if turn == 0 {
            board
        } else {
            [board[1], board[0]]
        }
    }
    fn apply_move(&self, src: i32, dst: i32) -> Self {
        [self[1], self[0] & !src.to_piece() | dst.to_piece()]
    }
    fn is_win_state(&self) -> Option<bool> {
        let dxdy = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, -1),
            (-1, 1),
        ];
        let check = |id: usize| -> bool {
            if self[1 - id].count_ones() == 1 {
                return true;
            }
            let mut set = DisjointSet { pre: [-1; 64] };
            let my: Vec<i32> = (0..64)
                .filter(|pos| (self[id] & pos.to_piece()) != 0)
                .collect();
            for pos in my.iter() {
                let mut flag = false;
                let (x0, y0) = pos.to_coord_2d();
                for (dx, dy) in dxdy.iter() {
                    let (x1, y1) = (x0 + dx, y0 + dy);
                    if x1 >= 0 && y1 >= 0 && x1 < 8 && y1 < 8 {
                        let p = (x1, y1).to_coord();
                        if (self[id] & p.to_piece()) != 0 {
                            flag = true;
                            set.join(*pos, p);
                        }
                    }
                }
                if !flag {
                    return false;
                }
            }
            set.is_joint(&my)
        };

        let win = check(0);
        let lose = check(1);

        if !win && !lose {
            None
        } else {
            Some(win)
        }
    }
    fn gen_rand_move(&self, turn: i32, rng: &mut RngProvider) -> (i32, i32) {

        let mut all_moves = gen_all_moves(&self, turn);
        let mut v: Vec<(i32, i32)> = vec![];

        while let GeneratorState::Yielded((src, dst)) = Pin::new(&mut all_moves).resume() {
            v.push((src, dst))
        }

        let len = v.len() as i32;

        if len > 0 {
            let rn: f32 = rng.gen();
            let rid = (len as f32 * rn).floor();
            v[rid as usize]
        } else {
            EMPTY_MOVE
        }

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
        (self & 0x7, self.overflowing_shr(3).0)
    }
    fn to_piece(self) -> u64 {
        1u64.overflowing_shl(self as u32).0
    }
    fn higher_eq(self) -> u64 {
        (self.to_piece() as i64).overflowing_neg().0 as u64
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
        self.0 | (self.1.overflowing_shl(3).0)
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
            let col = COL.overflowing_shl(x as u32).0;
            let col_cnt = (tot & col).count_ones() as i32;

            let top = pos + (col_cnt.overflowing_shl(3).0);
            let bottom = pos - (col_cnt.overflowing_shl(3).0);
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
            let row = ROW.overflowing_shl(y.overflowing_shl(3).0 as u32).0;
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
                SLASH0
                    .overflowing_shr((7 - pl).overflowing_shl(3).0 as u32)
                    .0
            } else {
                SLASH0
                    .overflowing_shl((pl - 7).overflowing_shl(3).0 as u32)
                    .0
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
                SLASH1.overflowing_shr(pl.overflowing_shl(3).0 as u32).0
            } else {
                SLASH1.overflowing_shl(-pl.overflowing_shl(3).0 as u32).0
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
                while let GeneratorState::Yielded(dst) = Pin::new(&mut moves).resume() {
                    yield (pos, dst)
                }
            }
        }
    }
}

#[derive(Debug)]
struct SearchNode {
    curr_move: (i32, i32),
    data: Option<(SearchNodeData, f32, f32)>,
}

#[derive(Debug)]
enum SearchNodeData {
    Mid {
        curr: i32,
        full: bool,
        board: [u64; 2],
        childs: Vec<SearchNode>,
    },
    Term(bool),
}

impl SearchNode {

    fn expand(&mut self, board: &[u64; 2], debug: bool) {

        if let None = self.data {
            let (src, dst) = self.curr_move;
            let new_board = board.apply_move(src, dst);
            self.data = Some((SearchNodeData::from(&new_board), 0f32, 0f32));
        }

    }

    fn find_max(&mut self) -> Option<*mut SearchNode> {

        let mut max_node: Option<*mut SearchNode> = None;
        let (ref mut data, _a0, b0) = self.data.as_mut().unwrap();
        match data {
            SearchNodeData::Mid { ref mut childs, .. } => {
                let mut max_fact = 0f32;
                for child in childs.iter_mut() {
                    if child.data.is_some() {
                        let (.., a1, b1) = child.data.as_ref().unwrap();

                        let fact = 1f32 - a1 / b1 + C * (b0.ln() / b1).sqrt();

                        if fact > max_fact {
                            max_fact = fact;
                            max_node = Some(child);
                        }
                    }

                }
            }
            SearchNodeData::Term(_win) => {} //
        }

        max_node

    }

    fn select(
        &mut self,
        board: &[u64; 2],
        debug: bool,
    ) -> (&mut Self, [u64; 2], Vec<&mut SearchNode>) {
        unsafe {
            let mut node_ptr: *mut SearchNode = self; // curr state node
            let mut board_ptr: *const [u64; 2] = board; // prev state board
            let mut path: Vec<&mut SearchNode> = vec![];

            loop {
                path.push(&mut *node_ptr); // add curr state

                match (*node_ptr).data {
                    // if this node is compact, select it.
                    None => return (&mut *node_ptr, *board_ptr, path),
                    // select from child nodes
                    Some((
                        SearchNodeData::Mid {
                            ref mut curr,
                            ref mut full,
                            ref mut childs,
                            ref board, // board of current state
                            ..
                        },
                        ..
                    )) => {
                        if !*full {
                            // select a child
                            let len = childs.len() as i32;
                            let selected = &mut childs[*curr as usize];
                            *curr += 1;
                            if *curr == len {
                                *full = true;
                                *curr = 0;
                            }

                            let selected_ptr: *mut SearchNode = selected;

                            // add curr to path
                            path.push(&mut *selected_ptr);
                            return (selected, *board, path);

                        } else {
                            // from this state
                            board_ptr = &*board;
                            node_ptr = (*node_ptr).find_max().unwrap();
                        }
                    }
                    // win node
                    Some((SearchNodeData::Term(win), ..)) => {
                        // this node
                        return (&mut *node_ptr, *board_ptr, path);
                    }
                }
            }
        }
    }

    fn simulate(&self, rng: &mut RngProvider, debug: bool) -> Option<bool> {

        let (ref data, ..) = self.data.as_ref().unwrap();

        match data {
            SearchNodeData::Mid { ref board, .. } => {
                let mut curr_board: [u64; 2] = *board;

                for step in 0..MAX_STEP {
                    // if debug {
                    //     alert(&format!("0> {:?}", curr_board));
                    // }

                    match curr_board.is_win_state() {
                        Some(win) => {

                            // if debug {
                            //     alert(&format!("1> {:?}", win));
                            // }

                            return Some(win ^ ((step & 1) != 0));
                        }
                        _ => {
                            // if debug {
                            //     alert(&format!("2> {:?}", curr_board));
                            // }

                            let (src, dst) = curr_board.gen_rand_move(0, rng);
                            curr_board = curr_board.apply_move(src, dst);
                        }
                    }
                }
                None
            }
            SearchNodeData::Term(win) => {
                // if debug {
                //     alert(&format!("3> {:?}", *self));
                // }
                return Some(*win);
            }
        }
    }

    fn back_propagate(&mut self, win: bool) {

        let (.., ref mut a, ref mut b) = self.data.as_mut().unwrap();

        *b += 1f32;
        if win {
            *a += 1f32;
        }

    }

}

impl SearchNodeData {

    fn from(board: &[u64; 2]) -> Self {

        if let Some(win) = board.is_win_state() {
            // alert(&format!("board = {:?}", board));
            return SearchNodeData::Term(win);
        }

        let mut all_moves_vec = vec![];
        let mut all_moves = gen_all_moves(board, 0);

        while let GeneratorState::Yielded((src, dst)) = Pin::new(&mut all_moves).resume() {
            all_moves_vec.push(SearchNode {
                curr_move: (src, dst),
                data: None,
            })
        }

        let len = all_moves_vec.len();

        SearchNodeData::Mid {
            board: *board,
            childs: all_moves_vec,
            curr: 0,
            full: len == 0,
        }
    }
}

fn mcts_search_pass(
    root: &mut SearchNode,
    board: &[u64; 2],
    rng: &mut RngProvider,
    mut debug: bool,
) -> bool {

    if debug {
        log(&format!("mcts started"))
    }

    let (curr_node, curr_board, mut path) = root.select(board, debug);

    let mut winn: bool = false;
    let mut res: bool = false;

    let (src, dst) = curr_node.curr_move;

    if debug {
        log(&format!(
            "selected node: {:?}, board: {:?}, from {:?} to {:?}",
            curr_node,
            curr_board,
            src.to_coord_2d(),
            dst.to_coord_2d()
        ))
    }

    // debug ||=
    // src == (5, 5).to_coord() && dst == (6, 6).to_coord();

    curr_node.expand(&curr_board, debug);

    if debug {
        log(&format!("expanded node: {:?}", curr_node))
    }

    if let Some(win) = curr_node.simulate(rng, debug) {
        winn = win;
        res = true;
    }

    if debug {
        log(&format!(
            "simulated node: finish: {:?} win: {:?}",
            res, winn
        ))
    }

    for node in path.iter_mut().rev() {
        node.back_propagate(winn);
        winn = !winn;
    }

    res
}


#[wasm_bindgen]
pub fn my_plain_solution(turn: i32, sparse: &[i32]) -> Move {

    let board = <[u64; 2]>::from_sparse_board(sparse, turn);
    let mut rng = wasm_rng();
    // let mut rng = FakeRng::new(1024);
    let mut root = SearchNode {
        curr_move: EMPTY_MOVE,
        data: Some((SearchNodeData::from(&board), 0f32, 0f32)),
    };

    // alert(&format!("{:?}", board));

    let mut cnt: i32 = 0;
    let all = MAX_NODE;
    for _i in 0..all {
        if mcts_search_pass(&mut root, &board, &mut rng, false) {
            cnt += 1;
        }
    }

    log(&format!("{} hits of {}", cnt, all));

    unsafe {
        if let Some(ptr) = root.find_max() {
            let (src, dst) = (*ptr).curr_move;
            let (x0, y0) = src.to_coord_2d();
            let (x1, y1) = dst.to_coord_2d();
            return Move { x0, y0, x1, y1 };
        } else {
            panic!()
        }
    }
}

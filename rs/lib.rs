#![feature(generators, generator_trait)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
use serde::ser::{self, Impossible, Serialize, SerializeMap, SerializeStruct, Serializer};

extern crate wbg_rand;
use wbg_rand::{wasm_rng, Rng, WasmRng};

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

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
    fn log_tree(x: &JsValue);
}

const ROW: u64 = 0xFFu64;
const COL: u64 = 0x0101010101010101u64;
const SLASH0: u64 = 0x0102040810204080u64;
const SLASH1: u64 = 0x8040201008040201u64;
const C: f32 = 1.414f32 * 3e-1;

// const MAX_STEP: usize = 512;
// const MAX_NODE: usize = 128;
const MAX_STEP: usize = 256;
const MAX_NODE: usize = 8196;

const EMPTY_MOVE: (i32, i32) = (100, 100);

const RNG_PHASE: u32 = 8;
const RNG_SHAMT: u32 = 8;
const RNG_MASK: u32 = (1u32 << RNG_SHAMT) - 1;

const FAKE_RNG_LEN: usize = 1 << 8;
const FAKE_RNG_VALUES: [u64; FAKE_RNG_LEN] = [
    1676761424638520202,
    2939895356340538458,
    17728998469973128227,
    4873731339148202259,
    14888766267554033333,
    3397190989610969540,
    17016120907035480532,
    5720051754036445887,
    9226006198283208365,
    4018310429034958582,
    12932234442750983849,
    1561155504280919192,
    13898158795496324787,
    5715089962962972963,
    1079756092685224277,
    7898850948263649175,
    5921989576063733925,
    16504417086336376473,
    4240952514127547152,
    13892607808829003127,
    9913266533438410728,
    12171393552861043390,
    3757904714689556701,
    8799141626483857099,
    15112476009282929885,
    12056093053672559491,
    14997789461864109048,
    6279772964424601346,
    3308669982062096412,
    16994103614101904274,
    18413575972931516964,
    10392610048028876503,
    18086827500338154459,
    4608828808289729298,
    14061891413303642155,
    16288506097374620300,
    11517521245459201017,
    12151372652580226307,
    9235720325026900645,
    7433823344142084156,
    6571722528565114074,
    18313037719398340516,
    14957875857370919346,
    2497512302106918145,
    2604686494671587446,
    3508265945822672164,
    14403833058561485841,
    7049299856786905296,
    2163235560049824351,
    13838320476001757569,
    17695574168459992,
    16680976756210633072,
    13098763226352090362,
    9491239379368690248,
    1963545842821512784,
    9903313772432692403,
    16728806772645202475,
    13738229821777006309,
    6334718664342137367,
    11536767335736030956,
    15364385787576022026,
    15910285379767651386,
    10066837523738016639,
    11785278039674685990,
    30956770070838388,
    5471559825518861608,
    552543316267517069,
    13615649174442901335,
    3766134670152004356,
    2032874038834920947,
    18225546607226973706,
    17182677365428888235,
    14152803445638057208,
    5265703220483582023,
    3220387302988544660,
    2606273476983589983,
    9149205774268822573,
    10955844135931433785,
    2193109761341758610,
    3404059774305765462,
    707381209765997804,
    8895733017833558147,
    16120688406502153905,
    2768260120132862439,
    12472295333486694327,
    13963903900262325044,
    7452253811085463924,
    940048371681649676,
    3209322651065194433,
    9793723033965714244,
    12510740029007402601,
    16227047529442877283,
    4303459429779168975,
    393000640965451993,
    11986793946204985917,
    12802571012373768498,
    3141959344722538689,
    6919762220100225400,
    10964311322043469355,
    3511652053681691308,
    844570772520343791,
    404117342785637878,
    7580705639920345507,
    8181479297670164124,
    2144102592099176944,
    6300201431409044071,
    8317076033381107516,
    4665244675622328608,
    6051988362316902582,
    15824127020801728259,
    5583106237902089642,
    12909847312603412095,
    13153371221269165188,
    14308666500783577712,
    1559139881421591123,
    9610260245941585539,
    17110392190811749880,
    5938100763269148410,
    4623769724401529823,
    10029177695292702044,
    2258689528387236946,
    2095433011696979013,
    11171207816933134132,
    9936139159715991650,
    13283674792752983853,
    4245403495737788709,
    10225615215342202676,
    13589633007058903406,
    477038050453562113,
    2157577791336328351,
    15039045358678315873,
    3181686648079685452,
    1711047192750385117,
    11840595698236494011,
    652530361805992006,
    1235398577916225669,
    5544741489090107245,
    8954026480920709355,
    3904546910116376838,
    14122053271804872947,
    1197998269244372091,
    11640542567916952696,
    13106655145976716629,
    3307454261830868710,
    12852845119402631043,
    8741307047886469612,
    6258499328920920952,
    17128120147247571144,
    9490803404556694735,
    1290107018526767447,
    15563807295763094433,
    3240736257704658728,
    7784419869505931903,
    12831339338296753344,
    5724645739582991588,
    5059088715799566884,
    2255478945871969242,
    6864669823810819636,
    18093549901541662719,
    10025586063300516853,
    10690778138020981677,
    17124894572556081551,
    9063309290606003608,
    14135282153307452154,
    16918576790600213037,
    1406440462984585053,
    17548341056499848913,
    1491564791923457376,
    7910795817404609775,
    18016257755022730265,
    14053186136624006838,
    8345490850787587698,
    14972524401579471279,
    1346546449563778970,
    4520688261236579144,
    15509445197340960776,
    5608540929518366295,
    12388752117392234366,
    680602229358171668,
    18429351960167797626,
    4716689556165444143,
    11671962534772679628,
    1394210961703736585,
    13175217261993577715,
    3320535918747679969,
    8173034598410817480,
    16089642607628774139,
    1254673656779780414,
    16014920622290801524,
    12223474832482448005,
    3751115562845375103,
    252653078596294799,
    11817532774757893581,
    6568795886591900286,
    1545590643709047235,
    10078078057595492494,
    3469276706308673442,
    11345670332764466485,
    11166722010043560901,
    11231306468591108878,
    8387053425303657539,
    16530469037217536487,
    5115453863221368819,
    11215048310092129523,
    285072285072496416,
    18158150010293132677,
    15944440813467023751,
    16225020464807301890,
    3015221707980462495,
    16427170926547517685,
    7746194321968066461,
    3611413298331730877,
    14854393899312352040,
    2046394030286510469,
    6115642871432519136,
    17283079244241396190,
    1064212136418418429,
    2384707396599419629,
    3465969274424065281,
    6654147952765748330,
    1753795585333893179,
    14843680762529132267,
    1778859148707539532,
    915900562075528480,
    13735947414674269624,
    12153294834607201986,
    3705577077924774982,
    17233341745947383405,
    9306836491618306671,
    1666746922907925419,
    12270798747298758276,
    1503963713510928470,
    6874113541115680784,
    11782526596220135258,
    1783061059702411232,
    11616125168708785948,
    11397644361446777381,
    146103362511728521,
    16260865860763507098,
    16220466138079195235,
    13097422310252406661,
    16167328130239623222,
    5022862876868630699,
    8600129242113030533,
    7742615801448773175,
    1481756419612638737,
    16314200924770581972,
    11925725101382370362,
    15924520858787191462,
    5980704901846371251,
    790112647235428245,
    16089511475710765047,
    172160118696180607,
    39550878273559962,
    8397461583374611443,
    3546525223732628155,
];

trait U64Rng {
    fn next(&mut self) -> u64;
    fn new() -> Self;
}

impl U64Rng for WasmRng {
    fn next(&mut self) -> u64 {
        self.next_u64()
    }
    fn new() -> Self {
        wasm_rng()
    }
}

struct MyRng<T: U64Rng> {
    curr: u64,
    phase: u32,
    rng: T,
}

struct FakeRng {
    phase: usize,
}

impl U64Rng for FakeRng {
    fn next(&mut self) -> u64 {
        self.phase = (self.phase + 1) & (FAKE_RNG_LEN - 1);
        FAKE_RNG_VALUES[self.phase]
    }
    fn new() -> Self {
        FakeRng {
            phase: WasmRng::new().next_u32() as usize & (FAKE_RNG_LEN - 1),
        }
    }
}

impl<T: U64Rng> MyRng<T> {
    fn next_u32(&mut self) -> u32 {
        if self.phase == 0 {
            self.curr = self.rng.next();
        }
        self.phase = (self.phase + 1) & (RNG_PHASE - 1);
        let val = self.curr as u32 & RNG_MASK;
        self.curr >>= RNG_SHAMT;
        val
    }
    fn new() -> Self {
        MyRng {
            curr: 0,
            phase: 0,
            rng: T::new(),
        }
    }
}

type RngProvider = MyRng<FakeRng>;

struct DisjointSet {
    pre: [i32; 64],
}

impl DisjointSet {
    fn join(&mut self, mut a: i32, mut b: i32) {
        a = self.get(a);
        b = self.get(b);
        if a != b {
            unsafe {
                *self.pre.get_unchecked_mut(b as usize) = a;
            }
        }
    }
    fn get(&mut self, mut a: i32) -> i32 {
        unsafe {
            let mut k = a;
            while *self.pre.get_unchecked_mut(a as usize) != -1 {
                a = *self.pre.get_unchecked_mut(a as usize)
            }
            while k != a {
                let j = *self.pre.get_unchecked_mut(k as usize);
                *self.pre.get_unchecked_mut(k as usize) = a;
                k = j;
            }
        }
        a
    }
    fn is_joint(&mut self, iter: impl Iterator<Item = i32>) -> bool {
        let mut u = iter.map(|x| self.get(x));
        if let Some(e) = u.next() {
            u.all(|x| x == e)
        } else {
            true
        }
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
            for pos in (0..64).filter(|pos| (self[id] & pos.to_piece()) != 0) {
                let mut flag = false;
                let (x0, y0) = pos.to_coord_2d();
                for (dx, dy) in dxdy.iter() {
                    let (x1, y1) = (x0 + dx, y0 + dy);
                    if x1 >= 0 && y1 >= 0 && x1 < 8 && y1 < 8 {
                        let p = (x1, y1).to_coord();
                        if (self[id] & p.to_piece()) != 0 {
                            flag = true;
                            set.join(pos, p);
                        }
                    }
                }
                if !flag {
                    return false;
                }
            }
            set.is_joint((0..64).filter(|pos| (self[id] & pos.to_piece()) != 0))
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

        let rn: i32 = rng.next_u32() as i32 & 0x3fi32;
        let b = self[turn as usize];

        // random piece from rn
        for i in (rn..64).filter(|i| (i.to_piece() & b) != 0) {
            let mut moves = gen_moves(&self, turn, i);
            if let GeneratorState::Yielded(dst) = Pin::new(&mut moves).resume() {
                return (i, dst);
            }
        }
        for i in (0..rn).filter(|i| (i.to_piece() & b) != 0) {
            let mut moves = gen_moves(&self, turn, i);
            if let GeneratorState::Yielded(dst) = Pin::new(&mut moves).resume() {
                return (i, dst);
            }
        }

        EMPTY_MOVE
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
pub struct SearchNode {
    pub curr_move: (i32, i32),
    pub data: Option<(SearchNodeData, f32, f32)>,
}

#[derive(Debug)]
pub enum SearchNodeData {
    Mid {
        curr: i32,
        full: bool,
        board: [u64; 2],
        childs: Vec<SearchNode>,
    },
    Term(bool),
}

impl Serialize for SearchNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some((ref data, a, b)) = self.data {
            let mut state = serializer.serialize_struct("node", 3)?;
            if self.curr_move == EMPTY_MOVE {
                state.serialize_field("move", "empty");
            } else {
                state.serialize_field(
                    "move",
                    &format!(
                        "{:?} -> {:?}",
                        self.curr_move.0.to_coord_2d(),
                        self.curr_move.1.to_coord_2d(),
                    ),
                );
            };
            state.serialize_field("value", &format!("{} / {} = {}", a, b, a as f32 / b as f32));
            state.serialize_field("detail", &data);

            state.end()
        } else {
            serializer.serialize_none()
        }
    }
}

impl Serialize for SearchNodeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {

        match self {
            SearchNodeData::Mid { ref childs, .. } => {
                let mut state = serializer.serialize_struct("node", 3)?;
                state.serialize_field(
                    "child",
                    &childs
                        .iter()
                        .filter(|x| x.data.is_some())
                        .collect::<Vec<&SearchNode>>(),
                );
                state.end()
            }
            SearchNodeData::Term(win) => serializer.serialize_bool(*win),
        }
    }
}

#[derive(Debug)]
struct MCTSSearchPass {
    hit: bool,
    win: bool,
    expand_term: bool,
    expand_depth: i32,
    simulate_depth: i32,
}

impl SearchNode {

    fn expand(&mut self, board: &[u64; 2], debug: bool) -> bool {

        if let None = self.data {
            let (src, dst) = self.curr_move;
            let new_board = board.apply_move(src, dst);
            self.data = Some((SearchNodeData::from(&new_board), 0f32, 0f32));
        }

        if let Some((SearchNodeData::Term(..), ..)) = self.data {
            true
        } else {
            false
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
    ) -> (Option<(&mut Self, [u64; 2])>, Vec<&mut SearchNode>) {
        unsafe {
            let mut node_ptr: *mut SearchNode = self; // curr state node
            let mut board_ptr: *const [u64; 2] = board; // prev state board
            let mut path: Vec<&mut SearchNode> = vec![];

            loop {
                path.push(&mut *node_ptr); // add curr state

                match (*node_ptr).data {
                    // if this node is compact, select it.
                    None => return (Some((&mut *node_ptr, *board_ptr)), path),
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
                            return (Some((selected, *board)), path);

                        } else {
                            // from this state
                            board_ptr = &*board;
                            if let Some(new_node_ptr) = (*node_ptr).find_max() {
                                node_ptr = new_node_ptr;
                            } else {
                                return (None, path);
                            }
                        }
                    }
                    // win node
                    Some((SearchNodeData::Term(win), ..)) => {
                        // this node
                        return (Some((&mut *node_ptr, *board_ptr)), path);
                    }
                }
            }
        }
    }

    fn simulate(&self, rng: &mut RngProvider, debug: bool) -> (Option<bool>, i32) {

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

                            return (Some(win ^ ((step & 1) != 0)), step as i32);
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
                (None, MAX_STEP as i32)
            }
            SearchNodeData::Term(win) => {
                // if debug {
                //     alert(&format!("3> {:?}", *self));
                // }
                return (Some(*win), 0);
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
    debug: bool,
) -> MCTSSearchPass {

    if debug {
        log(&format!("mcts started"))
    }

    let (sel_res, mut path) = root.select(board, debug);
    let mut winn: bool = false;
    let mut res: bool = false;
    let mut expand_term = false;
    let expand_depth = path.len() as i32;
    let mut simulate_depth = 0;

    if let Some((curr_node, curr_board)) = sel_res {

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

        expand_term = curr_node.expand(&curr_board, debug);

        if debug {
            log(&format!("expanded node: {:?}", curr_node))
        }

        let (simulate_res, simulate_dep) = curr_node.simulate(rng, debug);
        simulate_depth = simulate_dep;

        if let Some(win) = simulate_res {
            winn = win;
            res = true;
        }

        if debug {
            log(&format!(
                "simulated node: finish: {:?} win: {:?}",
                res, winn
            ))
        }

    } else {
        if debug {
            log(&format!("failed to select"))
        }
    }

    for node in path.iter_mut().rev() {
        node.back_propagate(winn);
        winn = !winn;
    }

    MCTSSearchPass {
        hit: res,
        win: !winn,
        expand_term,
        expand_depth,
        simulate_depth,
    }
}


#[wasm_bindgen]
pub fn my_plain_solution(turn: i32, sparse: &[i32]) -> Move {

    let board = <[u64; 2]>::from_sparse_board(sparse, turn);
    let mut rng = MyRng::new();
    // let mut rng = FakeRng::new(1024);
    let mut root = SearchNode {
        curr_move: EMPTY_MOVE,
        data: Some((SearchNodeData::from(&board), 0f32, 0f32)),
    };

    // alert(&format!("{:?}", board));

    let mut hit_cnt: i32 = 0;
    let mut expand_term_cnt: i32 = 0;

    let mut total_simulate_depth: i32 = 0;
    let mut max_simulate_depth: i32 = 0;
    let mut min_simulate_depth: i32 = MAX_STEP as i32;

    let mut total_expand_depth: i32 = 0;
    let mut max_expand_depth: i32 = 0;
    let mut min_expand_depth: i32 = MAX_STEP as i32;

    let all = MAX_NODE;
    for _i in 0..all {

        let MCTSSearchPass {
            hit,
            win,
            expand_term,
            simulate_depth,
            expand_depth,
        } = mcts_search_pass(&mut root, &board, &mut rng, false);
        if hit {
            hit_cnt += 1;
        }
        if expand_term {
            expand_term_cnt += 1;
        }

        total_expand_depth += expand_depth;
        max_expand_depth = max_expand_depth.max(expand_depth);
        min_expand_depth = min_expand_depth.min(expand_depth);

        total_simulate_depth += simulate_depth;
        max_simulate_depth = max_simulate_depth.max(simulate_depth);
        min_simulate_depth = min_simulate_depth.min(simulate_depth);
    }

    log(&format!("{} hits of {}", hit_cnt, all));
    log(&format!("{} term expansion of {}", expand_term_cnt, all));

    log(&format!(
        "{} average expand depth",
        total_expand_depth as f32 / all as f32
    ));
    log(&format!("{} max expand depth", max_expand_depth));
    log(&format!("{} min expand depth", min_expand_depth));

    log(&format!(
        "{} average simulate depth",
        total_simulate_depth as f32 / all as f32
    ));
    log(&format!("{} max simulate depth", max_simulate_depth));
    log(&format!("{} min simulate depth", min_simulate_depth));

    // if let Ok(value) = JsValue::from_serde(&root) {
    //     log_tree(&value);
    // } else {
    //     log("failed to serilize");
    // }

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

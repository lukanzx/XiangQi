use crate::component::piece::{Kind, Piece, Side};
use crate::public::{Pos, ROUTE_OFFSET};
use crate::{chess, player};
use bevy::prelude::*;
use chessai::position;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameMode {
    AiGame,
    DeduceGame,
    InterGame,
}

#[derive(Resource)]
pub struct Data {
    // 红色方玩家
    pub white_player: player::Player,
    // 黑色方玩家
    pub black_player: player::Player,
    // 棋盘地图
    pub broad_map: [[Option<Piece>; 9]; 10],
    // 当前回合数
    pub round: usize,
    // 没有吃子的步数
    pub noeat_move_num: usize,
    // 当前行棋方
    pub current_side: Option<Side>,
    // 游戏引擎
    pub engine: chessai::Engine,
    // 状态变化记录
    pub mode: Option<GameMode>,
    // 选择的棋子
    pub selected: Option<Piece>,
    // 游戏模式
    pub ai_side: Option<Side>,
}

impl Data {
    pub fn new() -> Self {
        info!("init system data");
        Self {
            engine: chessai::Engine::new(),
            selected: None,
            white_player: player::Player::new_white(),
            black_player: player::Player::new_black(),
            broad_map: [
                [
                    Some(Piece::white(Kind::Rook, 0, 0)),
                    Some(Piece::white(Kind::Knight, 0, 1)),
                    Some(Piece::white(Kind::Bishop, 0, 2)),
                    Some(Piece::white(Kind::Advisor, 0, 3)),
                    Some(Piece::white(Kind::King, 0, 4)),
                    Some(Piece::white(Kind::Advisor, 0, 5)),
                    Some(Piece::white(Kind::Bishop, 0, 6)),
                    Some(Piece::white(Kind::Knight, 0, 7)),
                    Some(Piece::white(Kind::Rook, 0, 8)),
                ],
                [None, None, None, None, None, None, None, None, None],
                [
                    None,
                    Some(Piece::white(Kind::Cannon, 2, 1)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::white(Kind::Cannon, 2, 7)),
                    None,
                ],
                [
                    Some(Piece::white(Kind::Pawn, 3, 0)),
                    None,
                    Some(Piece::white(Kind::Pawn, 3, 2)),
                    None,
                    Some(Piece::white(Kind::Pawn, 3, 4)),
                    None,
                    Some(Piece::white(Kind::Pawn, 3, 6)),
                    None,
                    Some(Piece::white(Kind::Pawn, 3, 8)),
                ],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [
                    Some(Piece::black(Kind::Pawn, 6, 0)),
                    None,
                    Some(Piece::black(Kind::Pawn, 6, 2)),
                    None,
                    Some(Piece::black(Kind::Pawn, 6, 4)),
                    None,
                    Some(Piece::black(Kind::Pawn, 6, 6)),
                    None,
                    Some(Piece::black(Kind::Pawn, 6, 8)),
                ],
                [
                    None,
                    Some(Piece::black(Kind::Cannon, 7, 1)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::black(Kind::Cannon, 7, 7)),
                    None,
                ],
                [None, None, None, None, None, None, None, None, None],
                [
                    Some(Piece::black(Kind::Rook, 9, 0)),
                    Some(Piece::black(Kind::Knight, 9, 1)),
                    Some(Piece::black(Kind::Bishop, 9, 2)),
                    Some(Piece::black(Kind::Advisor, 9, 3)),
                    Some(Piece::black(Kind::King, 9, 4)),
                    Some(Piece::black(Kind::Advisor, 9, 5)),
                    Some(Piece::black(Kind::Bishop, 9, 6)),
                    Some(Piece::black(Kind::Knight, 9, 7)),
                    Some(Piece::black(Kind::Rook, 9, 8)),
                ],
            ],
            round: 0,
            noeat_move_num: 0,
            current_side: None,
            mode: None,
            ai_side: None,
        }
    }

    pub fn get_last_move(&self) -> Option<(Pos, Pos)> {
        match self.engine.mv_list.last() {
            Some(mv) => {
                let ((src_row, src_col), (dst_row, dst_col)) = position::move2pos(*mv);
                println!("{src_row}-{src_col} {dst_row}-{dst_col}");
                Some((Pos::new(src_row, src_col), Pos::new(dst_row, dst_col)))
            }
            None => None,
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for pieces in self.broad_map.iter() {
            let mut line = String::new();
            let mut num = 0;
            for piece in pieces {
                if let Some(piece) = piece {
                    if num > 0 {
                        line.push_str(&num.to_string());
                        num = 0;
                    }
                    line.push_str(&piece.code());
                } else {
                    num += 1;
                }
            }
            if num > 0 {
                line.push_str(&num.to_string());
            }
            fen.push_str(&line);
            fen.push_str("/")
        }
        fen.pop().unwrap();
        fen.push_str(&format!(
            " {} -- {} {}",
            self.current_side.unwrap().code(),
            self.noeat_move_num,
            self.round
        ));
        fen
    }

    pub fn get_current_player(&mut self) -> &mut player::Player {
        match self.current_side.unwrap() {
            Side::Black => &mut self.black_player,
            Side::White => &mut self.white_player,
        }
    }

    /// 换边
    pub fn change_side(&mut self) {
        match self.current_side.unwrap() {
            Side::White => {
                self.round += 1;
                self.current_side = Some(Side::Black);
            }
            Side::Black => {
                self.current_side = Some(Side::White);
            }
        }
    }

    pub fn parse_route(&self, route: String) -> ((usize, usize), (usize, usize)) {
        let bytes = route.as_bytes();
        let src_col = (bytes[0] - ROUTE_OFFSET.0) as usize;
        let src_row = (bytes[1] - ROUTE_OFFSET.1) as usize;
        let dst_col = (bytes[2] - ROUTE_OFFSET.0) as usize;
        let dst_row = (bytes[3] - ROUTE_OFFSET.1) as usize;
        return ((src_row, src_col), (dst_row, dst_col));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_route() {
        // [97, 48, 105, 57]
        let test_str = String::from("a0i9");
        let ((row, col), (dst_row, dst_col)) = Data::new().parse_route(test_str);
        assert_eq!((row, col), (0, 0));
        assert_eq!((dst_row, dst_col), (9, 8));
    }

    #[test]
    fn test_match() {
        let n = 9;
        match n {
            1 => {
                println!("1")
            }
            9 => {
                println!("9")
            }
            _ => {
                println!("n")
            }
        }
    }
}

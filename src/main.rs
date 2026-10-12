extern crate datafrog;
use datafrog::{Iteration, Relation, Variable};

use std::fs::File;
use std::io::Write;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(PartialEq, Eq, PartialOrd,Ord)]
enum Color{
    Black,
    White,
}
//derive for compare by rust
#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd,Ord)]
enum PieceId {
    White_Pawn1,
    White_Pawn2,
    White_Pawn3,
    White_Pawn4,
    White_Pawn5,
    White_Pawn6,
    White_Pawn7,
    White_Pawn8,
    White_King,
    White_Queen,
    White_Bishop1,
    White_Bishop2,
    White_Knight1,
    White_Knight2,
    White_Rook1,
    White_Rook2,

    Black_Pawn1,
    Black_Pawn2,
    Black_Pawn3,
    Black_Pawn4,
    Black_Pawn5,
    Black_Pawn6,
    Black_Pawn7,
    Black_Pawn8,
    Black_King,
    Black_Queen,
    Black_Bishop1,
    Black_Bishop2,
    Black_Knight1,
    Black_Knight2,
    Black_Rook1,
    Black_Rook2,
}

struct Piece {
    id: PieceId,
    piece_type: PieceType,
    color: Color,      // 0: white, 1: black
}

struct Piece_Linked_List{
    piece_head: PieceId,
    pieve_tail: PieceId,
}

struct Cell_Num{
    cell_num: u8,
}

struct Step_State{
    step_current: u8,
    step_next: u8,
}

struct Board_State{
    step: u8,
    piece_id: PieceId,
    x: u8,
    y:u8,
}

struct Move_State{
    step_from: u8,
    piece_id: PieceId,
    x_from: u8,
    y_from:u8,
    step_to: u8,
    piece_id_to: PieceId,
    x_to: u8,
    y_to: u8,
}

struct One_Step{
    x_from: u8,
    y_from: u8,
    dx: i8,
    dy: i8,
    x_to: u8,
    y_to: u8,
}

// get the path to checkmate
// edges: (from, step, to, rook_id)
fn main() -> Result<(), Box<dyn std::error::Error>>{
let mut iteration = Iteration::new();

let mut cell_num_vec = Vec::new();
for i in 1..=8 {
    cell_num_vec.push(Cell_Num {
        cell_num: i,
    });
}
let mut horizontal_facts = Vec::new();
let mut vertical_facts = Vec::new();
let mut diagonal_facts = Vec::new();

let mut one_step_vec= Vec::<One_Step>::new();
let board_size: u8 = 8;
for x in 1..=board_size{
    for y in 1..=board_size{
        for dx in -1_i8..=1 {
            for dy in -1_i8..=1 {
                // 出発地点から進む方向 (dx, dy) が決まる
                if dx == 0 && dy == 0 {
                    continue;
                }
                // 移動先を計算
                let next_x = i16::from(x) + i16::from(dx);
                let next_y = i16::from(y) + i16::from(dy);
                let limit = i16::from(board_size);

                // 移動先が盤内か確認する
                if next_x >= 1 && next_x <= limit
                    && next_y >= 1 && next_y <= limit
                {
                    // 6フィールドを設定してpushする
                    one_step_vec.push(One_Step {
                        x_from: x,
                        y_from: y,
                        dx,
                        dy,
                        x_to: next_x as u8,
                        y_to: next_y as u8,
                    });
                }
            }
        }
    }
}

// (x_from, y_from, dx, dy), (x_to, y_to)
for one_step in one_step_vec {
    let fact = (
        (one_step.x_from, one_step.y_from, one_step.dx, one_step.dy),
        (one_step.x_to, one_step.y_to),
    );

    if one_step.dx != 0 && one_step.dy == 0 {
        horizontal_facts.push(fact);
    } else if one_step.dx == 0 && one_step.dy != 0 {
        vertical_facts.push(fact);
    } else if one_step.dx != 0 && one_step.dy != 0 {
        diagonal_facts.push(fact);
    }
}

// ((3, 3,  1, 0), (4, 3)) // Xを+1、Yはそのまま
// ((3, 3, -1, 0), (2, 3)) // Xを-1、Yはそのまま
let horizontal_onestep = Relation::from(horizontal_facts);
// ((3, 3, 0,  1), (3, 4)) // Yを+1、Xはそのまま
// ((3, 3, 0, -1), (3, 2)) // Yを-1、Xはそのまま
let vertical_onestep = Relation::from(vertical_facts);
// ((3, 3,  1,  1), (4, 4)) // Xを+1、Yを+1
// ((3, 3, -1, -1), (2, 2)) // Xを-1、Yを-1
let diagonal_onestep = Relation::from(diagonal_facts);


let initinal_board_state_vec = vec![
    Board_State { step: 0, piece_id: PieceId::White_Pawn1, x: 1, y: 2 },
    Board_State { step: 0, piece_id: PieceId::White_Pawn2, x: 2, y: 2 },
    Board_State { step: 0, piece_id: PieceId::White_Pawn3, x: 3, y: 2 },
    Board_State { step: 0, piece_id: PieceId::White_Pawn4, x: 4, y: 2 },
    Board_State { step: 0, piece_id: PieceId::White_Pawn5, x: 5, y: 2 },
    Board_State { step: 0, piece_id: PieceId::White_Pawn6, x: 6, y: 2 },
    Board_State { step: 0, piece_id: PieceId::White_Pawn7, x: 7, y: 2 },
    Board_State { step: 0, piece_id: PieceId::White_Pawn8, x: 8, y: 2 },

    Board_State { step: 0, piece_id: PieceId::White_Rook1,   x: 1, y: 1 },
    Board_State { step: 0, piece_id: PieceId::White_Knight1, x: 2, y: 1 },
    Board_State { step: 0, piece_id: PieceId::White_Bishop1, x: 3, y: 1 },
    Board_State { step: 0, piece_id: PieceId::White_Queen,   x: 4, y: 1 },
    Board_State { step: 0, piece_id: PieceId::White_King,    x: 5, y: 1 },
    Board_State { step: 0, piece_id: PieceId::White_Bishop2, x: 6, y: 1 },
    Board_State { step: 0, piece_id: PieceId::White_Knight2, x: 7, y: 1 },
    Board_State { step: 0, piece_id: PieceId::White_Rook2,   x: 8, y: 1 },

    Board_State { step: 0, piece_id: PieceId::Black_Pawn1, x: 1, y: 7 },
    Board_State { step: 0, piece_id: PieceId::Black_Pawn2, x: 2, y: 7 },
    Board_State { step: 0, piece_id: PieceId::Black_Pawn3, x: 3, y: 7 },
    Board_State { step: 0, piece_id: PieceId::Black_Pawn4, x: 4, y: 7 },
    Board_State { step: 0, piece_id: PieceId::Black_Pawn5, x: 5, y: 7 },
    Board_State { step: 0, piece_id: PieceId::Black_Pawn6, x: 6, y: 7 },
    Board_State { step: 0, piece_id: PieceId::Black_Pawn7, x: 7, y: 7 },
    Board_State { step: 0, piece_id: PieceId::Black_Pawn8, x: 8, y: 7 },

    Board_State { step: 0, piece_id: PieceId::Black_Rook1,   x: 1, y: 8 },
    Board_State { step: 0, piece_id: PieceId::Black_Knight1, x: 2, y: 8 },
    Board_State { step: 0, piece_id: PieceId::Black_Bishop1, x: 3, y: 8 },
    Board_State { step: 0, piece_id: PieceId::Black_Queen,   x: 4, y: 8 },
    Board_State { step: 0, piece_id: PieceId::Black_King,    x: 5, y: 8 },
    Board_State { step: 0, piece_id: PieceId::Black_Bishop2, x: 6, y: 8 },
    Board_State { step: 0, piece_id: PieceId::Black_Knight2, x: 7, y: 8 },
    Board_State { step: 0, piece_id: PieceId::Black_Rook2,   x: 8, y: 8 },
];

let pieces_vec = vec![
    Piece { id: PieceId::White_Pawn1, piece_type: PieceType::Pawn, color: Color::White },
    Piece { id: PieceId::White_Pawn2, piece_type: PieceType::Pawn, color: Color::White },
    Piece { id: PieceId::White_Pawn3, piece_type: PieceType::Pawn, color: Color::White },
    Piece { id: PieceId::White_Pawn4, piece_type: PieceType::Pawn, color: Color::White },
    Piece { id: PieceId::White_Pawn5, piece_type: PieceType::Pawn, color: Color::White },
    Piece { id: PieceId::White_Pawn6, piece_type: PieceType::Pawn, color: Color::White },
    Piece { id: PieceId::White_Pawn7, piece_type: PieceType::Pawn, color: Color::White },
    Piece { id: PieceId::White_Pawn8, piece_type: PieceType::Pawn, color: Color::White },

    Piece { id: PieceId::White_King, piece_type: PieceType::King, color: Color::White },
    Piece { id: PieceId::White_Queen, piece_type: PieceType::Queen, color: Color::White },

    Piece { id: PieceId::White_Bishop1, piece_type: PieceType::Bishop, color: Color::White },
    Piece { id: PieceId::White_Bishop2, piece_type: PieceType::Bishop, color: Color::White },

    Piece { id: PieceId::White_Knight1, piece_type: PieceType::Knight, color: Color::White },
    Piece { id: PieceId::White_Knight2, piece_type: PieceType::Knight, color: Color::White },

    Piece { id: PieceId::White_Rook1, piece_type: PieceType::Rook, color: Color::White },
    Piece { id: PieceId::White_Rook2, piece_type: PieceType::Rook, color: Color::White },

    Piece { id: PieceId::Black_Pawn1, piece_type: PieceType::Pawn, color: Color::Black },
    Piece { id: PieceId::Black_Pawn2, piece_type: PieceType::Pawn, color: Color::Black },
    Piece { id: PieceId::Black_Pawn3, piece_type: PieceType::Pawn, color: Color::Black },
    Piece { id: PieceId::Black_Pawn4, piece_type: PieceType::Pawn, color: Color::Black },
    Piece { id: PieceId::Black_Pawn5, piece_type: PieceType::Pawn, color: Color::Black },
    Piece { id: PieceId::Black_Pawn6, piece_type: PieceType::Pawn, color: Color::Black },
    Piece { id: PieceId::Black_Pawn7, piece_type: PieceType::Pawn, color: Color::Black },
    Piece { id: PieceId::Black_Pawn8, piece_type: PieceType::Pawn, color: Color::Black },

    Piece { id: PieceId::Black_King, piece_type: PieceType::King, color: Color::Black },
    Piece { id: PieceId::Black_Queen, piece_type: PieceType::Queen, color: Color::Black },

    Piece { id: PieceId::Black_Bishop1, piece_type: PieceType::Bishop, color: Color::Black },
    Piece { id: PieceId::Black_Bishop2, piece_type: PieceType::Bishop, color: Color::Black },

    Piece { id: PieceId::Black_Knight1, piece_type: PieceType::Knight, color: Color::Black },
    Piece { id: PieceId::Black_Knight2, piece_type: PieceType::Knight, color: Color::Black },

    Piece { id: PieceId::Black_Rook1, piece_type: PieceType::Rook, color: Color::Black },
    Piece { id: PieceId::Black_Rook2, piece_type: PieceType::Rook, color: Color::Black },
];

// TODO: make a script and input data instead of writing here.
let move_state_vec = vec![
    Move_State { step_from: 0, piece_id: PieceId::White_Pawn1, x_from: 1, y_from: 2,step_to: 1, piece_id_to: PieceId::White_Pawn1,x_to: 1, y_to: 3 },
    Move_State { step_from: 1, piece_id: PieceId::Black_Pawn1, x_from: 1, y_from: 7,step_to: 2, piece_id_to: PieceId::Black_Pawn1,x_to: 1, y_to: 6 },
    Move_State { step_from: 2, piece_id: PieceId::White_Pawn2, x_from: 2, y_from: 2,step_to: 3, piece_id_to: PieceId::White_Pawn2,x_to: 2, y_to: 3 },
    Move_State { step_from: 3, piece_id: PieceId::Black_Pawn2, x_from: 2, y_from: 7,step_to: 4, piece_id_to: PieceId::Black_Pawn2,x_to: 2, y_to: 6 },
];

let step_state_vec = vec![
    Step_State{step_current:0, step_next:1},
    Step_State{step_current:1, step_next:2},
    Step_State{step_current:2, step_next:3},
];

// Vec<Cell_Num> -> Vec<u8> -> Relation<u8>
// let cell_numbers: Vec<u8> = cell_num_vec
//     .into_iter()
//     .map(|cell| cell.cell_num)
//     .collect();
// Relation. for stable facts and dont add new facts while 推論
let step_variable: Variable<(u8,u8)> = iteration.variable::<(u8, u8)>("step_state");

let mut piece_linked_list_vec = Vec::new();
for i in 1..pieces_vec.len(){
    piece_linked_list_vec.push((pieces_vec[i-1].id,pieces_vec[i].id));
} 
//最後から最初へのリンクを追加する
if let (Some(first), Some(last)) =
    (pieces_vec.first(), pieces_vec.last())
{
    piece_linked_list_vec.push((last.id, first.id));
}
let piece_linked_list: Vec<(PieceId, PieceId)> = piece_linked_list_vec
    .into_iter()
    .map(|p| (
        p.0,
        p.1,
    ))
    .collect();
let piece_linked_list_relation: Relation<(PieceId, PieceId)> = piece_linked_list.into();

let pieces: Vec<(PieceId, PieceType, Color)> = pieces_vec
    .into_iter()
    .map(|p| (
        p.id,
        p.piece_type,
        p.color,
    ))
    .collect();
let pieces_relation: Relation<(PieceId, PieceType, Color)> = pieces.into();

let mut cell_num_linked_list_vec = Vec::new();
for i in 1..cell_num_vec.len(){
    cell_num_linked_list_vec.push((cell_num_vec[i-1].cell_num,cell_num_vec[i].cell_num));
    cell_num_linked_list_vec.push((cell_num_vec[cell_num_vec.len()-i].cell_num, 
                                    cell_num_vec[cell_num_vec.len()-i-1].cell_num));
} 
let cell_num_linked_list: Vec<(u8, u8)> = cell_num_linked_list_vec
    .into_iter()
    .map(|c| (
        c.0,
        c.1,
    ))
    .collect();
let cell_num_linked_list_relation: Relation<(u8, u8)> = cell_num_linked_list.into();

// get from input
let step_vec:Vec<Step_State> = Vec::new();
let step: Vec<(u8,u8)> = step_vec
    .into_iter()
    .map(|steps_state| (steps_state.step_current, steps_state.step_next))
    .collect();

//get from input
let move_state: Vec<_> 
    = move_state_vec
    .into_iter() // aquire elements Board_State from Vec
    .map(|move_state| ((move_state.step_from, 
        move_state.piece_id, 
        move_state.x_from, move_state.y_from),
        (move_state.step_to,
        move_state.piece_id_to,
        move_state.x_to, move_state.y_to))) // convert elements to state
    .collect(); // collect converted results as Vec<(u8, PieceId, u8, u8)
let moved_state_relation: Relation<((u8, PieceId, u8, u8), (u8, PieceId, u8, u8))> = move_state.into(); 

// join with board_state ((step, PID),(x,y))
let move_state_intermidiate:
    Relation<((u8, PieceId), ())> =
    moved_state_relation
        .iter()
        .map(|&((step, piece_id, _, _), _)| {
            ((step, piece_id), ())
        })
        .collect();

// Variable. for dynamic facts that are generated additionlay while 推論中
let initinal_board_state_vec: Vec<((u8, PieceId), (u8, u8))> 
    = initinal_board_state_vec
    .into_iter() // aquire elements Board_State from Vec
    .map(|state| ((state.step, state.piece_id), (state.x, state.y))) // convert elements to state
    .collect(); // collect converted results as Vec<((u8, PieceId, u8, u8),())>
let board_state_variable = 
iteration.variable::<((u8, PieceId), (u8, u8))>("board_state");
board_state_variable.insert(initinal_board_state_vec.into());

let join_boardState_movedIntermidiate:
    Variable<((u8, PieceId, u8, u8), ())> =
    iteration.variable("join_boardState_movedIntermidiate");

let unmoved_cursor_variable:
    Variable<(PieceId, (u8, u8, usize))> =
    iteration.variable("join_unmoved_cursor");
unmoved_cursor_variable.insert(
    moved_state_relation
        .iter()
        .map(|&((step_from, piece_id, _, _), (step_to, _, _, _))| {
            (piece_id, (step_from, step_to, 0usize))
        })
        .collect(),
);

let join_unmovedCursor_pieceLinkedList:
    Variable<(usize, (u8, u8, PieceId))> =
    iteration.variable(
        "join_unmovedCursor_pieceLinkedList",
    );

// 駒数からコピー回数を生成
let piece_count = pieces_relation.elements.len();

let copyCount_relation:
    Relation<(usize, usize)> =
    (0..piece_count.saturating_sub(1))
        .map(|count| (count, count + 1))
        .collect();

let join_unmovedCursorPieceLinkedList_copyCount:
    Variable<((u8, PieceId), (u8, usize))> =
    iteration.variable(
        "join_unmovedCursorPieceLinkedList_copyCount",
    );

// (x_from, y_from, dx, dy), (x_to, y_to)
// ((step_from、piece_id、x_from、y_from), (step_to、piece_id_to、x_to、y_to))
let piece_path: Variable<((u8, u8, i8, i8), ((u8, PieceId, u8, u8), (u8, PieceId, u8, u8)))> 
    = iteration.variable("piece_path");

while iteration.changed() {
    // Rules

    // increasing reference, use VariableVariable
    // generate moved piece's next step's board state
    moved_piece_change(&board_state_variable, &move_state_intermidiate, &join_boardState_movedIntermidiate, &moved_state_relation);
    
    unmoved_piece_change(
    &board_state_variable,
    &unmoved_cursor_variable,
    &piece_linked_list_relation,
    &copyCount_relation,
    &join_unmovedCursor_pieceLinkedList,
    &join_unmovedCursorPieceLinkedList_copyCount,
);    
}
// print updated board_state_variable
// TODO: only the moved piece info is added into new board_state_variable.
// I have to add all other unmoved pieces location into new board_state_variable too
println!("{:?}", board_state_variable.complete().elements);
// step, pieceId, x,y

Ok(())
}

fn moved_piece_change(
    board_state_variable:
        &Variable<((u8, PieceId), (u8, u8))>,
    move_state_intermidiate:
        &Relation<((u8, PieceId), ())>,
    join_boardState_movedIntermidiate:
        &Variable<((u8, PieceId, u8, u8), ())>,
    moved_state_relation: &Relation<(
        (u8, PieceId, u8, u8),
        (u8, PieceId, u8, u8),
    )>,
) {
    // 現在の盤面から、動く駒の座標を取得
    join_boardState_movedIntermidiate.from_join(
        board_state_variable,
        move_state_intermidiate,
        |&(step_from, piece_id),
         &(x_from, y_from), &()| {
            (
                (step_from, piece_id, x_from, y_from),
                (),
            )
        },
    );

    // 移動元が一致するMoveから次の盤面を生成
    board_state_variable.from_join(
        join_boardState_movedIntermidiate,
        moved_state_relation,
        |_, 
            &(), 
            &(step_to, piece_id_to, x_to, y_to)| {
                ((step_to, piece_id_to), (x_to, y_to))
            },
    );
}

fn unmoved_piece_change(
    board_state_variable:
        &Variable<((u8, PieceId), (u8, u8))>,
    unmoved_cursor_variable:
        &Variable<(PieceId, (u8, u8, usize))>,
    piece_linked_list_relation:
        &Relation<(PieceId, PieceId)>,
    copyCount_relation:
        &Relation<(usize, usize)>,
    join_unmovedCursor_pieceLinkedList:
        &Variable<(usize, (u8, u8, PieceId))>,
    join_unmovedCursorPieceLinkedList_copyCount:
        &Variable<((u8, PieceId), (u8, usize))>,
) {
    // リンクから次のコピー候補を選ぶ
    join_unmovedCursor_pieceLinkedList.from_join(
        unmoved_cursor_variable,
        piece_linked_list_relation,
        |_, &(step_from, step_to, counter),
         &piece_tail| {
            (counter, (step_from, step_to, piece_tail))
        },
    );

    // コピー回数の上限を適用
    join_unmovedCursorPieceLinkedList_copyCount
        .from_join(
            join_unmovedCursor_pieceLinkedList,
            copyCount_relation,
            |_, &(step_from, step_to, piece_tail),
             &copy_count_next| {
                (
                    (step_from, piece_tail),
                    (step_to, copy_count_next),
                )
            },
        );

    // 座標を次のステップへコピー
    board_state_variable.from_join(
        join_unmovedCursorPieceLinkedList_copyCount,
        board_state_variable,
        |&(_, piece_id), &(step_to, _),
         &(x_from, y_from)| {
            ((step_to, piece_id), (x_from, y_from))
        },
    );

    // 次の駒へ進むカーソルを生成
    unmoved_cursor_variable.from_map(
        join_unmovedCursorPieceLinkedList_copyCount,
        |&((step_from, piece_id),
           (step_to, copy_count_next))| {
            (
                piece_id,
                (step_from, step_to, copy_count_next),
            )
        },
    );
}

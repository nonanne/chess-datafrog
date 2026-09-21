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
    Move_State { step_from: 1, piece_id: PieceId::Black_Pawn1, x_from: 1, y_from: 7,step_to: 2, piece_id_to: PieceId::White_Pawn1,x_to: 1, y_to: 6 },
    Move_State { step_from: 2, piece_id: PieceId::White_Pawn2, x_from: 2, y_from: 2,step_to: 3, piece_id_to: PieceId::White_Pawn1,x_to: 2, y_to: 3 },
    Move_State { step_from: 3, piece_id: PieceId::Black_Pawn2, x_from: 2, y_from: 7,step_to: 4, piece_id_to: PieceId::White_Pawn1,x_to: 2, y_to: 6 },
];
// let step_state_vec = vec![
//     Step_State{step_current:0,step_to}
// ];

// Vec<Cell_Num> -> Vec<u8> -> Relation<u8>
let cell_numbers: Vec<u8> = cell_num_vec
    .into_iter()
    .map(|cell| cell.cell_num)
    .collect();
// Relation. for stable facts and dont add new facts while 推論
let cell_num_relation: Relation<u8> = cell_numbers.into();

let pieces: Vec<(PieceId, PieceType, Color)> = pieces_vec
    .into_iter()
    .map(|p| (
        p.id,
        p.piece_type,
        p.color,
    ))
    .collect();
let pieces_relation: Relation<(PieceId, PieceType, Color)> = pieces.into();

// get from input
let step_vec:Vec<Step_State> = Vec::new();
let step: Vec<(u8,u8)> = step_vec
    .into_iter()
    .map(|steps_state| (steps_state.step_current, steps_state.step_next))
    .collect();

//get from input
let move_state: Vec<_> = move_state_vec
    .into_iter() // aquire elements Board_State from Vec
    .map(|move_state| ((move_state.step_from, 
                        move_state.piece_id, 
                        move_state.x_from, move_state.y_from),
                        (move_state.step_to,
                        move_state.piece_id_to,
                        move_state.x_to, move_state.y_to))) // convert elements to state
    .collect(); // collect converted results as Vec<(u8, PieceId, u8, u8)
let move_state_relation: Relation<((u8, PieceId, u8, u8), (u8, PieceId, u8, u8))> = move_state.into(); 

// Variable. for dynamic facts that are generated additionlay while 推論中
let initinal_board_state_vec: Vec<((u8, PieceId, u8, u8),())> = initinal_board_state_vec
    .into_iter() // aquire elements Board_State from Vec
    .map(|state| ((state.step, state.piece_id, state.x, state.y),())) // convert elements to state
    .collect(); // collect converted results as Vec<((u8, PieceId, u8, u8),())>
let board_state_variable = iteration.variable::<((u8, PieceId, u8, u8),())>("board_state");
board_state_variable.insert(initinal_board_state_vec.into());



while iteration.changed() {
    // Rules
    // increasing reference, use VariableVariable
    // insert into board_state_variable
    board_state_variable.from_join(
    &board_state_variable, &move_state_relation,
    |_place
            ,&(), &(step_to,piece_id,x_to,y_to)|
            {((step_to,piece_id,x_to,y_to),())});

}
// print updated board_state_variable
// TODO: only the moved piece info is added into new board_state_variable.
// I have to add all other unmoved pieces location into new board_state_variable too
println!("{:?}", board_state_variable.complete().elements);

Ok(())
}

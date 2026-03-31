extern crate datafrog;
use datafrog::{Iteration, Relation};

use std::fs::File;
use std::io::Write;

enum PieceType {
    King = 1,
    Queen = 2,
    Rook = 3,
    Pown = 4,
}

struct Piece {
    id: u8,
    piece_type: u8, // 1: King, 2: Queen, 3: Rook, 4: Pown
    color: u8,      // 0: white, 1: black
    pos: u8,
}

// Convert board position (1..64) to (row, col)
fn row_column_tuple(pos: u8) -> (u8, u8) {
    let position_row = pos / 8;
    let position_col = pos % 8;
    return (position_row, position_col);
}

// Build all rook moves on an empty 8x8 board.
// rook_move(from, to)
fn build_rook_moves() -> Vec<(u8, u8)> {
    let mut moves = Vec::new();

    for from in 0..=63 {
        let (row_from, col_from) = row_column_tuple(from);

        for to in 0..=63 {
            if from == to {
                continue;
            }

            let (row_to, col_to) = row_column_tuple(to);
            // make available rook moves facts
            if row_from == row_to || col_from == col_to {
                moves.push((from, to));
            }
        }
    }
    return moves;
}

// successor(step, next_step)
fn build_successor(max_step: u8) -> Vec<(u8, u8)> {
    let mut successors = Vec::new();

    for step in 0..=max_step -1{
        let next_step = step + 1;
        successors.push((step, next_step));
    }
    return successors;
}

// allowed_step(step)
fn build_allowed_steps(max_step: u8) -> Vec<(u8,)> {
    let mut allowed_steps = Vec::new();

    for step in 0..=max_step {
        allowed_steps.push((step,));
    }
    return allowed_steps;
}

// get the path to checkmate
// edges: (from, step, to, rook_id)
fn extract_path(
    edges: &[(u8, u8, u8, u8)], // list of (from, step, to, rook_id)
    target_pos: u8, // the position of king
    reached_step: u8, // how many steps it took to checkmate
) -> Vec<u8> {
    let mut path = vec![];
    path.push(target_pos);
    let mut current = target_pos;
    let mut step = reached_step;

    while step > 0 {
        let mut all_edges: Option<(u8, u8, u8, u8)> = None;

        for &(from, s, to, rid) in edges {
            if s == step && to == current{
                all_edges = Some((from, s, to, rid));
                break;
            }
        }
        if all_edges.is_some()
        {
            let from = all_edges.unwrap().0;
            path.push(from);
            current = from;
        }
        step -= 1;  
    }

    path.reverse();
    return path;
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let max_step: u8 = 5;

    let pieces_vec = vec![
        Piece { id: 0, piece_type: PieceType::King as u8,  color: 0, pos: 0},  // white king
        Piece { id: 1, piece_type: PieceType::Rook as u8,  color: 0, pos: 13},  // white rook
        Piece { id: 5, piece_type: PieceType::Rook as u8,  color: 0, pos: 16},  // white rook
        Piece { id: 4, piece_type: PieceType::Rook as u8,  color: 1, pos: 29},  // black rook
        Piece { id: 2, piece_type: PieceType::Queen as u8, color: 0, pos: 20},  // white queen
        Piece { id: 3, piece_type: PieceType::King as u8,  color: 1, pos: 51},  // black king
    ];

    let rook_move_vec: Vec<(u8, u8)> = build_rook_moves();     // (from, to)
    let successor_step_vec: Vec<(u8, u8)> = build_successor(max_step);        // (step, next_step)

    // position of black king
    let target_pos: u8 = pieces_vec
        .iter()
        .find(|p| p.piece_type == PieceType::King as u8 && p.color == 1)
        .map(|p| p.pos)
        .expect("no black king");

    // rook_pos_facts(pos, step, rook_id)
    let rook_at_initial: Vec<(u8, (u8, u8))> = pieces_vec
        .iter()
        .filter(|p| p.piece_type == PieceType::Rook as u8 && p.color == 0) 
        .map(|p| (p.pos, (0, p.id))) // (pos, step, rook_id)
        .collect();
    /////
    // static facts
    /////
    let rook_move_facts: Relation<(u8, u8)> = rook_move_vec.into();
    let successor_step_facts: Relation<(u8, u8)> = successor_step_vec.into();
    let target_facts: Relation<(u8, ())> = vec![(target_pos, ())].into();

    /////
    // dynamic facts
    /////
    let mut iteration = Iteration::new();
    // rook_pos_facts(pos, step, rook_id)
    let rook_pos_facts = iteration.variable::<(u8, (u8, u8))>("rook_pos_facts");
    // rook_step_facts(step, (pos, rook_id))
    let rook_step_facts =
        iteration.variable::<(u8, (u8, u8))>("rook_step_facts");
    // rook_pos_next_facts(pos, (next_step, rook_id))
    let rook_pos_next_facts =
        iteration.variable::<(u8, (u8, u8))>("rook_pos_next_facts");

    // move_edge(from, step, to, rook_id)
    let move_edge = iteration.variable::<(u8, u8, u8, u8)>("move_edge");

    /////
    // goal facts
    // checkmates(step, rook_id)
    /////
    let checkmates = iteration.variable::<(u8, u8)>("checkmates");

    // initial load
    rook_pos_facts.insert(rook_at_initial.into());

    while iteration.changed() {
        /////
        // rules
        /////

        /////
        // rook_pos_facts(pos, step, rook_id)
        // => rook_step_facts(step, (pos, rook_id))
        /////
        rook_step_facts.from_map(
            &rook_pos_facts,
            |&(pos, (step, rook_id))| 
            (step, (pos, rook_id)),
        );

        /////
        // rook_step_facts(step, (pos, rook_id))
        // + successor_step_facts(step, next_step)
        // => rook_pos_next_facts(pos, (next_step, rook_id))
        /////
        rook_pos_next_facts.from_join(
            &rook_step_facts,
            &successor_step_facts,
            |_step, &(pos, rook_id), &next_step| 
            (pos, (next_step, rook_id)),
        );

        /////
        // rook_pos_next_facts(from, (next_step, rook_id))
        // + rook_move_facts(from, to)
        // => rook_pos_facts(to, next_step, rook_id)
        /////
        rook_pos_facts.from_join(
            &rook_pos_next_facts,
            &rook_move_facts,
            |_from, &(next_step, rook_id), &to| 
            (to, (next_step, rook_id)),
        );

        /////
        // rook_pos_next_facts(from, (next_step, rook_id))
        // + rook_move_facts(from, to)
        // => move_edge(from, step, to, rook_id)
        /////
        move_edge.from_join(
            &rook_pos_next_facts,
            &rook_move_facts,
            |&from, &(next_step, rook_id), &to| 
            (from, next_step.saturating_sub(1), to, rook_id),
        );

        /////
        // rook_at_by_pos_facts(pos, (step, rook_id))
        // + target_facts(pos)
        // => checkmates(step, rook_id)
        /////
        checkmates.from_join(
            &rook_pos_facts,
            &target_facts,
            |_pos, &(step, rook_id), &()| 
            (step, rook_id),
        );
    }

    // Results
    let reached_results = checkmates.complete();
    let move_edge_results = move_edge.complete();

    println!("***** Result *****");
    if let Some(&(step, rook_id)) = reached_results.elements.first() {
        println!(
            "Checkmate！ rook_id={} reached in {}steps to pos={} .",
            rook_id, step, target_pos
        );

        let path = extract_path(&move_edge_results.elements, target_pos, step);

        let json_data = format!(
            "{{\n  \"path\": {:?},\n  \"target\": {},\n  \"steps\": {}\n}}",
            path, target_pos, step
        );

        let mut file = File::create("input.json")?;
        file.write_all(json_data.as_bytes())?;
        println!("[Success] input.json is generated (For Circom witness)");
    } else {
        println!("[Failed]: Cannot reach in {}steps", max_step);
    }
    Ok(())
}
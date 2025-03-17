use crate::Board;
use crate::board_utils;
use std::cmp;


pub fn minimax(board: &mut Board, depth: i32, mut alpha: i32, mut beta: i32) -> Result<[i32; 2], String> {
    let mut best_move: i32 = 0; // Placeholder value until best move is found.
    let ending = board.evaluate_pos(); // When the end of the game is reached, we want to return the outcome of that game
    if ending != 2 {                        // so we can cascade it back up the game tree to find the expected outcome.
        return Ok([ending, 8]); 
    }

    if board.playertomove {
        let mut max_eval = -10000;
        for amove in board_utils::generate_moves(board.xbb | board.obb) {
            board.mv(amove)?;
            let eval = minimax(&mut Board {..*board}, depth + 1, alpha, beta).unwrap()[0];
            board.unmv();
            if eval > max_eval {
                max_eval = eval;
                best_move = amove;
            }
            alpha = cmp::max(alpha, eval);
            if beta <= alpha {
                break; 
            }
                 

        } 
        return Ok([max_eval, best_move])
    }
    else {
        let mut min_eval = 10000;
        for amove in board_utils::generate_moves(board.xbb | board.obb) {
            board.mv(amove)?;
            let eval = minimax(&mut Board {..*board}, depth + 1, alpha, beta).unwrap()[0];
            board.unmv();
            if eval < min_eval {
                min_eval = eval;
                best_move = amove;
                 
            }
            
            beta = cmp::min(beta, eval);
            if beta <= alpha {
                break; 
            }
        } 

        return Ok([min_eval, best_move])
    }
    
    
}

use std::io;
use std::io::Write;
use crate::Board;
use crate::ai;
use colored::Colorize;

pub fn generate_moves(bb: i32) -> Vec<i32> {
    use crate::POWERS_OF_TWO;
    let mut output = Vec::new();
    for (index, pointer) in POWERS_OF_TWO.iter().rev().enumerate() {
        if pointer & bb != 0 {
            continue
        } else {
            output.push(index.try_into().unwrap());
        }

    }
    output
}

pub fn print_board(board: [i32; 9]) {
    println!();
    let mut row_ending = 2;
    for (index, pos)in board.iter().enumerate() {
        match pos {
            2 => print!("* "),
            1 => print!("X "),
            0 => print!("O "),
            _ => panic!("That's not good...")
        }
        if row_ending == index {
            row_ending += 3;
            println!()
        }

    } 
}

pub fn game_loop(mut board: &mut Board) -> Result<(), String> {

    println!(r"
  _______ _          _______             _______         
 |__   __(_)        |__   __|           |__   __|        
    | |   _  ___ ______| | __ _  ___ ______| | ___   ___ 
    | |  | |/ __|______| |/ _` |/ __|______| |/ _ \ / _ \
    | |  | | (__       | | (_| | (__       | | (_) |  __/
    |_|  |_|\___|      |_|\__,_|\___|      |_|\___/ \___|

    ________________________________________________________");
                                                         
                                                         

    println!("First, decide if you want to be X or O: ");
    let mut xoro = String::new();
 
    io::stdin()
        .read_line(&mut xoro)
        .expect("failed to read line");
    let isx = match xoro.as_str().trim() {
        "x" => true,
        "o" => false,
        _ => panic!("Please enter x or o! (uncapitalized)")
    };
    print!("{}", "Board moves are entered as numbers that correspond to a square. Here is the key:\n\n0 1 2\n3 4 5\n6 7 8\n".bright_green());


    while board.evaluate_pos() == 2 {

        println!("\n-------------------------------------------");
        if board.playertomove{
            if isx {
                human_move(&mut board)?
            } else {
                ai_move(&mut board)?
            }
        }

        if board.evaluate_pos() != 2 {
            break 
        }

        if !board.playertomove {
            if isx {
                ai_move(&mut board)?
            } else {
                human_move(&mut board)?
            } 
        }
    }
    match board.evaluate_pos() {
        1 => println!("X won!"),
        -1 => println!("O won!"),
        _ => println!("It's a tie!")
    }
    Ok(())
    
}




pub fn human_move(board: &mut Board) -> Result<(), String> {
     
    if board.playertomove {
        print!("It's X's turn to move. Please enter a move: ");
        io::stdout().flush().unwrap(); 
        let mut mov = String::new();
        io::stdin()
            .read_line(&mut mov)
            .expect("Failed to read line");
        let mov = mov.trim()
            .parse()
            .expect("Please enter a number!");

        if generate_moves(board.xbb | board.obb).iter().any(|&i| {i == mov}) {

            board.mv(mov.try_into().unwrap())?;
            print_board(board.generate_board()?);

            return Ok(())
        }
        println!("\n Invalid move, please try again");
    }
    
    if !board.playertomove {

        print!("It's O's turn to move. Please enter a move: ");
        io::stdout().flush().unwrap(); 
        let mut mov = String::new();
        io::stdin()
            .read_line(&mut mov)
            .expect("Failed to read line");
        let mov = mov.trim()
            .parse()
            .expect("Please enter a number!");

        if generate_moves(board.xbb | board.obb).iter().any(|&i| {i == mov}) {

            board.mv(mov.try_into().unwrap())?;
            print_board(board.generate_board()?);

            return Ok(())
        }
        println!("\n Invalid move, please try again");
    }
    Ok(())

}

pub fn ai_move(board: &mut Board) -> Result<(), String> {

    use std::time::Instant;
    println!("Ai's Move:");
    let now = Instant::now();
    let [ev, mov] = ai::minimax(board, 0, -100, 100).unwrap();
    let elapsed = now.elapsed();
    if generate_moves(board.xbb | board.obb).iter().any(|&i| {i == mov}) {

        board.mv(mov.try_into().unwrap())?;
        print_board(board.generate_board()?);
        println!("Ai predicted outcome of {ev} in {elapsed:?}");
        return Ok(()) 
    }
    println!("\n Invalid move, please try again");
    Ok(())

}

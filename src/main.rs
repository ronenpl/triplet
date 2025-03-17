



use trust::Board;
use trust::board_utils;

fn main() -> Result<(), String> {
    let mut board = Board::new();

    board_utils::game_loop(&mut board)?;
    Ok(())
}


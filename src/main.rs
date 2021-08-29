mod board;
mod snake;
mod utils;
use board::Board;
use crossterm::{self, Result};
use snake::Snake;

fn main() -> Result<()> {
    let mut out = utils::Out::new();
    let mut board = Board::new(60, 60);
    out.hide()?;
    board.render_border()?;
    let mut snake = Snake::new(&board);
    snake.tick()?;
    Ok(())
}

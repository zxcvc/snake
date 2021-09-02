mod board;
mod snake;
mod utils;

use board::Board;
use crossterm::{self, cursor, event, terminal, ExecutableCommand, Result};
use snake::Snake;

fn main() -> Result<()> {
    loop {
        let mut out = utils::Out::new();

        let mut board = Board::new(20, 20);
        out.hide()?;
        board.render_border()?;
        let mut snake = Snake::new(&board);
        snake.tick()?;
        println!("{}", r#"r：重新开始  e：退出"#);
        println!("\r");
        terminal::enable_raw_mode()?;

        loop {
            match event::read()? {
                event::Event::Key(e) => match e.code {
                    event::KeyCode::Char('r') => {
                        break;
                    }
                    event::KeyCode::Char('e') => return Ok(()),
                    _ => {}
                },
                _ => {}
            }
        }

        out.init()?;
    }
    Ok(())
}

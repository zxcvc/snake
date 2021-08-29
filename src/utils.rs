use crossterm::{self, cursor, style, terminal, ExecutableCommand, Result};
use lazy_static::lazy_static;
use std::{cell::RefCell, fmt::Display, io::Write, rc::Rc};

lazy_static! {
    static ref out: std::io::Stdout = std::io::stdout();
}
pub struct Out<'a> {
    pub out: Rc<RefCell<&'a std::io::Stdout>>,
}

impl<'a> Out<'a> {
    pub fn new() -> Self {
        Self {
            out: Rc::new(RefCell::new(&out)),
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        (*self.out)
            .borrow_mut()
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Result::Ok(())
    }

    pub fn print_by_position<T: Display>(&mut self, x: u16, y: u16, value: T) -> Result<()> {
        (*self.out)
            .borrow_mut()
            .execute(cursor::MoveTo(x, y))?
            .execute(style::Print(value))?
            .flush()?;
        Result::Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        (*self.out).borrow_mut().execute(cursor::RestorePosition)?;
        Ok(())
    }

    pub fn hide(&mut self) -> Result<()> {
        (*self.out).borrow_mut().execute(cursor::Hide)?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        (*self.out)
            .borrow_mut()
            .execute(cursor::Show)?
            .execute(cursor::RestorePosition)?;
        Ok(())
    }
}

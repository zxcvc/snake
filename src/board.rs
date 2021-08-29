use super::utils;
use crossterm::{self, cursor, style, ExecutableCommand, Result};
pub struct Board<'a> {
    pub width: u16,
    pub height: u16,
    out: utils::Out<'a>,
}

impl<'a> Board<'a> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            out: utils::Out::new(),
        }
    }

    pub fn render_border(&mut self) -> Result<()> {
        self.out.clear()?;
        for x in 0..self.width {
            self.out.print_by_position(x, 0, "# ")?;
            self.out.print_by_position(x, self.height, "# ")?;
        }

        for y in 1..self.height - 2 {
            self.out.print_by_position(0, y, "#")?;
            self.out.print_by_position(self.width, y, "#")?;
        }

        (*self.out.out)
            .borrow_mut()
            .execute(style::Print("\n"))?
            .execute(cursor::SavePosition)?;

        Ok(())
    }
}

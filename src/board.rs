use super::utils;
use crossterm::{self, cursor, style, ExecutableCommand, Result};
pub struct Board<'a> {
    pub width: u16,
    pub height: u16,
    pub left_top: (u16, u16),
    pub right_bottom: (u16, u16),
    out: utils::Out<'a>,
}

impl<'a> Board<'a> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width: width,
            height,
            left_top: (0, 0),
            right_bottom: (width * 2 - 2, height - 2),
            out: utils::Out::new(),
        }
    }

    pub fn render_border(&mut self) -> Result<()> {
        self.out.clear()?;
        let border_icon = "▫️";
        let (horizontal1, horizonta2) = (self.left_top.0, self.right_bottom.0);
        let (vertical1, vertical2) = (self.left_top.1, self.right_bottom.1);
        for x in horizontal1..=horizonta2 {
            self.out.print_by_position(x, 0, border_icon)?;
            self.out.print_by_position(x, vertical2, border_icon)?;
        }
        for y in vertical1..=vertical2 {
            self.out.print_by_position(0, y, border_icon)?;
            self.out.print_by_position(horizonta2, y, border_icon)?;
        }

        (*self.out.out)
            .borrow_mut()
            .execute(style::Print("\n\n"))?
            .execute(cursor::SavePosition)?;

        Ok(())
    }
}

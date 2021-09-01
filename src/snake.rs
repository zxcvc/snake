use super::board::Board;
use super::utils::rand_position_gen;
use super::utils::Out;
use crossterm::{event, terminal, Result};
use std::{collections::linked_list::LinkedList, time::Duration};

#[derive(PartialEq, Eq)]
enum Deriction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Snake<'a> {
    body: LinkedList<(u16, u16)>,
    out: Out<'a>,
    deriction: Deriction,
    food: (u16, u16),
    board: &'a Board<'a>,
}

impl<'a> Snake<'a> {
    pub fn new(board: &'a Board<'a>) -> Self {
        let mut body = LinkedList::new();
        body.push_back((20, 10));
        body.push_back((21, 10));
        body.push_back((22, 10));
        Self {
            body,
            out: Out::new(),
            deriction: Deriction::Left,
            food: (30, 30),
            board: board,
        }
    }

    pub fn render(&mut self) -> Result<()> {
        for &(x, y) in self.body.iter() {
            self.out.print_by_position(x, y, "●")?;
        }
        self.out.restore()?;
        Ok(())
    }

    pub fn next_position(&self, current: (u16, u16)) -> (u16, u16) {
        let x = match self.deriction {
            Deriction::Left => current.0 - 1,
            Deriction::Right => current.0 + 1,
            _ => current.0,
        };
        let y = match self.deriction {
            Deriction::Up => current.1 - 1,
            Deriction::Down => current.1 + 1,
            _ => current.1,
        };
        (x, y)
    }

    fn on_body(&self, position: &(u16, u16)) -> bool {
        for it in self.body.iter() {
            if it == position {
                return true;
            }
        }
        false
    }

    pub fn gen_food(&mut self) {
        loop {
            self.food = rand_position_gen(
                self.board.left_top.0 + 2..self.board.right_bottom.0 - 2,
                self.board.left_top.1 + 2..self.board.right_bottom.1 - 2,
            );
            if !self.on_body(&self.food) {
                break;
            }
        }
    }

    pub fn rend_food(&mut self) -> Result<()> {
        self.out.print_by_position(self.food.0, self.food.1, "*")?;
        Ok(())
    }

    pub fn eat_food(&mut self) -> Result<()> {
        self.out.print_by_position(self.food.0, self.food.1, " ")?;
        let next = self.next_position(self.food);
        self.body.push_front(next);
        self.render()?;
        Ok(())
    }

    pub fn can_eat(&self) -> bool {
        // &self.food == self.body.front().unwrap()
        if let Some(x) = self.body.front() {
            return x == &self.food;
        } else {
            return false;
        }
    }

    pub fn is_collide(&self) -> bool {
        let head = *self.body.front().unwrap();
        for &it in self.body.iter().skip(1) {
            if head == it {
                return true;
            }
        }
        if head.0 <= self.board.left_top.0
            || head.1 <= self.board.left_top.1
            || head.0 >= self.board.right_bottom.0
            || head.1 >= self.board.right_bottom.1
        {
            return true;
        }
        false
    }

    pub fn tick(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        self.gen_food();
        self.rend_food()?;
        loop {
            self.rend_food()?;

            if event::poll(Duration::from_secs_f32(0.3))? {
                match event::read()? {
                    event::Event::Key(e) => match e.code {
                        event::KeyCode::Esc => {
                            self.out.init()?;
                            std::process::exit(0);
                        }
                        event::KeyCode::Up => {
                            if self.deriction != Deriction::Down {
                                self.deriction = Deriction::Up
                            }
                        }
                        event::KeyCode::Down => {
                            if self.deriction != Deriction::Up {
                                self.deriction = Deriction::Down
                            }
                        }
                        event::KeyCode::Left => {
                            if self.deriction != Deriction::Right {
                                self.deriction = Deriction::Left
                            }
                        }
                        event::KeyCode::Right => {
                            if self.deriction != Deriction::Left {
                                self.deriction = Deriction::Right
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            if let Some(&(x, y)) = self.body.back() {
                self.out.print_by_position(x, y, " ")?;
            }

            self.body.pop_back();
            let next = self.next_position(*self.body.front().unwrap());
            self.body.push_front(next);

            if self.can_eat() {
                self.eat_food()?;
                self.gen_food();
                self.rend_food()?;
            }

            self.render()?;
            if self.is_collide() {
                self.out.init()?;
                std::process::exit(0);
            }
        }
    }
}

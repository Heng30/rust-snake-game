use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Diretion {
    Up,
    Down,
    Left,
    Right,
}

impl Diretion {
    pub fn opposite(&self) -> Diretion {
        match *self {
            Diretion::Up => Diretion::Down,
            Diretion::Down => Diretion::Up,
            Diretion::Left => Diretion::Right,
            Diretion::Right => Diretion::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    diretion: Diretion,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            diretion: Diretion::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Diretion>) {
        match dir {
            Some(d) => self.diretion = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.diretion {
            Diretion::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Diretion::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Diretion::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Diretion::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Diretion {
        self.diretion
    }

    pub fn next_head(&self, dir: Option<Diretion>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.diretion;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Diretion::Up => (head_x, head_y - 1),
            Diretion::Down => (head_x, head_y + 1),
            Diretion::Left => (head_x - 1, head_y),
            Diretion::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            // 移动中，头尾相连的情况，不算失败
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}

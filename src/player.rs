use crate::WINDOW_HEIGHT;

pub const PLAYER_WIDTH: u32 = 20;
pub const PLAYER_HEIGHT: u32 = 120;

const UPPER_LIMIT: i32 = 0;
const LOWER_LIMIT: i32 = (WINDOW_HEIGHT - PLAYER_HEIGHT) as i32;
const STEP: i32 = 5;

pub struct Player {
    pub score: u16,
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new(x: i32) -> Self {
        Self {
            score: 0,
            x,
            y: (WINDOW_HEIGHT - PLAYER_HEIGHT) as i32 / 2,
        }
    }

    pub fn rectanlge(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x, self.y, PLAYER_WIDTH, PLAYER_HEIGHT)
    }

    pub fn up(&mut self) {
        if self.y > UPPER_LIMIT {
            self.y -= STEP;
        }
    }

    pub fn down(&mut self) {
        if self.y < LOWER_LIMIT {
            self.y += STEP;
        }
    }
}

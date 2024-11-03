use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const BALL_SIZE: u32 = 20;

const INITIAL_SPEED: f64 = 3.0;
const SPEED_FACTOR: f64 = 0.3;
const UPPER_LIMIT: i32 = 0;
const LOWER_LIMIT: i32 = (WINDOW_HEIGHT - BALL_SIZE / 2) as i32;

pub struct Ball {
    pub right: bool,
    pub up: bool,
    pub x: i32,
    pub y: i32,
    speed: f64,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            right: true,
            up: true,
            x: ((WINDOW_WIDTH - BALL_SIZE) / 2) as i32,
            y: ((WINDOW_HEIGHT - BALL_SIZE) / 2) as i32,
            speed: INITIAL_SPEED,
        }
    }

    pub fn rectanlge(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x, self.y, BALL_SIZE, BALL_SIZE)
    }

    pub fn update(&mut self) {
        let speed = self.speed.floor() as i32;
        self.x += if self.right { speed } else { -speed };
        self.y -= if self.up { speed } else { -speed };

        if self.y <= UPPER_LIMIT {
            self.up = false;
        } else if self.y >= LOWER_LIMIT {
            self.up = true;
        }
    }

    pub fn reset(&mut self, side: bool) {
        self.x = ((WINDOW_WIDTH - BALL_SIZE) / 2) as i32;
        self.y = ((WINDOW_HEIGHT - BALL_SIZE) / 2) as i32;
        self.speed = INITIAL_SPEED;
        self.right = side;
        self.up = true;
    }

    pub fn change_direction(&mut self) {
        self.right = !self.right;
        self.speed += SPEED_FACTOR;
    }
}

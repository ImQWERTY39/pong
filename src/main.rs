use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

mod player;
use player::Player;

mod ball;
use ball::Ball;

const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;

const WHITE: Color = Color::RGB(255, 255, 255);
const BLACK: Color = Color::RGB(0, 0, 0);

fn main() {
    let sdl_context = sdl2::init().expect("Shouldn't have failed");
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Pong", 1, 1)
        .fullscreen()
        // .position_centered()
        .build()
        .expect("Shouldn't have failed");

    let mut canvas = window.into_canvas().build().expect("Shouldn't have failed");
    let mut event_pump = sdl_context.event_pump().expect("huh?");
    // let mut audio_ctlr = sdl_context.audio().expect("huh?");

    let mut player1 = Player::new(30);
    let mut player2 = Player::new(WINDOW_WIDTH as i32 - 50);
    let mut ball = Ball::new();
    let mut paused = true;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => paused = !paused,
                _ => (),
            }
        }

        if !paused {
            event_pump
                .keyboard_state()
                .pressed_scancodes()
                .for_each(|key| match key {
                    Scancode::W => player1.up(),
                    Scancode::S => player1.down(),
                    Scancode::Up => player2.up(),
                    Scancode::Down => player2.down(),
                    _ => (),
                });

            update_game_state(&mut ball, &mut player1, &mut player2);
        }

        canvas.set_draw_color(BLACK);
        canvas.clear();

        draw_info(&mut canvas);
        draw_rect(&mut canvas, player1.rectanlge());
        draw_rect(&mut canvas, player2.rectanlge());
        draw_rect(&mut canvas, ball.rectanlge());

        canvas.present();

        std::thread::sleep(Duration::from_nanos(1_000_000_000 / 60));
    }
}

fn update_game_state(ball_: &mut Ball, player1: &mut Player, player2: &mut Player) {
    if ball_.x <= 0 {
        player2.score += 1;
        ball_.reset(true);
    }

    if ball_.x + ball::BALL_SIZE as i32 >= { WINDOW_WIDTH as i32 } {
        player1.score += 1;
        ball_.reset(false);
    }

    if ball_.x <= player1.x + player::PLAYER_WIDTH as i32
        && ball_.y + ball::BALL_SIZE as i32 > player1.y
        && ball_.y < player1.y + player::PLAYER_HEIGHT as i32
    {
        ball_.change_direction();
    }

    if ball_.x + ball::BALL_SIZE as i32 >= player2.x
        && ball_.y + ball::BALL_SIZE as i32 > player2.y
        && ball_.y < player2.y + player::PLAYER_HEIGHT as i32
    {
        ball_.change_direction();
    }

    ball_.update();
}

fn draw_info(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(WHITE);

    let center = { (WINDOW_WIDTH / 2) as i32 };
    let window_height = { WINDOW_HEIGHT as i32 };
    let gap = window_height / 64;

    let mut height = 0;
    while height < window_height {
        canvas
            .draw_line((center, height), (center, height + gap))
            .expect("HUH");
        height += gap * 2;
    }
}

fn draw_rect(canvas: &mut Canvas<Window>, player: sdl2::rect::Rect) {
    canvas.set_draw_color(WHITE);
    canvas.fill_rect(player).expect("how?");
}

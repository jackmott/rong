extern crate sdl2;
extern crate time;

use std::path::Path;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use std::thread;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Font;
use sdl2::render::TextureQuery;
use std::time::Duration;
use std::collections::HashSet;
use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const PADDLE_OFFSET: u32 = 20;
const PADDLE_WIDTH: i16 = 10;
const BALL_SIZE: i16 = 5;

enum GameState {
    Player1Ded,
    Player2Ded,
    NotDed,
}



macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

macro_rules! C {
    (WHITE) => (Color::RGB(255, 255, 255));
    (RED) => (Color::RGB(255,0,0));
}


fn get_current_millis() -> u64 {
    let timespec = time::get_time();
    let mut millis = timespec.sec as u64 * 1000;
    millis += timespec.nsec as u64 / 1_000_000;
    millis
}

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            //println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            //println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn draw_text(s: &str, font: &Font, canvas: &mut Canvas<Window>) {
    let texture_creator = canvas.texture_creator();
    let surface = font.render(s).blended(C!(RED)).unwrap();
    let tex = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let TextureQuery { width, height, .. } = tex.query();
    let target = get_centered_rect(width, height, SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    canvas.copy(&tex, None, Some(target)).unwrap();
}

struct Ball {
    x: f32,
    y: f32,
    xv: f32,
    yv: f32,
    color: Color,
}

struct Paddle {
    x: f32,
    y: f32,
    len: i32,
    color: Color,
}



fn main() {



    let num_players = 2;
    let mut game_state = GameState::NotDed;

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let window = video_subsys
        .window("rong", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let font_path = Path::new("oneway.ttf");
    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::STYLE_NORMAL);





    //let mut font = ttf_context.load_font()

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();


    let mut events = sdl_context.event_pump().unwrap();

    let mut ball = Ball {
        x: 100.0,
        y: 100.0,
        xv: 250.0,
        yv: 250.0,
        color: C!(WHITE),
    };
    let mut player = Paddle {
        x: PADDLE_OFFSET as f32,
        y: 300.0,
        len: 100,
        color: C!(WHITE),
    };
    let mut enemy = Paddle {
        x: (SCREEN_WIDTH - PADDLE_OFFSET) as f32,
        y: 100.0,
        len: 100,
        color: C!(WHITE),
    };

    let mut last_time = get_current_millis();
    'main: loop {
        match game_state {
            GameState::Player1Ded => draw_text("Player 1 Died", &font, &mut canvas),
            GameState::Player2Ded => draw_text("Player 2 Died", &font, &mut canvas),        
            GameState::NotDed => {

                let _ = canvas.filled_circle(ball.x as i16, ball.y as i16, BALL_SIZE, ball.color);
                let _ = canvas.box_(player.x as i16,
                                    player.y as i16 - (player.len / 2) as i16,
                                    player.x as i16 + PADDLE_WIDTH,
                                    player.y as i16 + (player.len / 2) as i16,
                                    player.color);
                let _ = canvas.box_(enemy.x as i16,
                                    enemy.y as i16 - (player.len / 2) as i16,
                                    enemy.x as i16 + PADDLE_WIDTH,
                                    enemy.y as i16 + (player.len / 2) as i16,
                                    enemy.color);
                let now = get_current_millis();

                let elapsed_seconds = (now - last_time) as f32 / 1000.0;
                last_time = now;

                //AI
                if num_players == 1 {
                    enemy.y = ball.y;
                }

                if ball.y <= 0.0 || ball.y >= SCREEN_HEIGHT as f32 {
                    ball.yv = -1.0 * ball.yv;
                    if ball.y <= 0.0 {
                        ball.y = 0.1
                    } else if ball.y >= SCREEN_HEIGHT as f32 {
                        ball.y = SCREEN_HEIGHT as f32 - 0.1;
                    }
                }
                if ball.x <= (PADDLE_OFFSET + PADDLE_WIDTH as u32) as f32 {
                    if ball.y < player.y + (player.len / 2) as f32 &&
                       ball.y > player.y - (player.len / 2) as f32 {
                        ball.xv = -1.0 * ball.xv;
                        ball.x = (PADDLE_OFFSET + PADDLE_WIDTH as u32) as f32 + 0.1;
                    } else {
                        game_state = GameState::Player1Ded;
                    }
                } else if ball.x >= (SCREEN_WIDTH - PADDLE_OFFSET) as f32 {
                    if ball.y < enemy.y + (enemy.len / 2) as f32 &&
                       ball.y > enemy.y - (enemy.len / 2) as f32 {
                        ball.xv = -1.0 * ball.xv;
                        ball.x = (SCREEN_WIDTH - PADDLE_OFFSET) as f32 - 0.1;
                    } else {
                        game_state = GameState::Player2Ded;
                    }
                }

                ball.x += ball.xv * elapsed_seconds;
                ball.y += ball.yv * elapsed_seconds;


                let keys: HashSet<Keycode> = events
                    .keyboard_state()
                    .pressed_scancodes()
                    .filter_map(Keycode::from_scancode)
                    .collect();


                if keys.contains(&Keycode::W) {
                    player.y -= 1000.0 * elapsed_seconds;
                } else if keys.contains(&Keycode::S) {
                    player.y += 1000.0 * elapsed_seconds;
                }

                if num_players == 2 {
                    if keys.contains(&Keycode::Up) {
                        enemy.y -= 1000.0 * elapsed_seconds;
                    } else if keys.contains(&Keycode::Down) {
                        enemy.y += 1000.0 * elapsed_seconds;
                    }
                }


            }
        }
        for event in events.poll_iter() {

            match event {

                Event::Quit { .. } => break 'main,

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                    else if keycode == Keycode::Space {
                        ball = Ball {
                            x: 100.0,
                            y: 100.0,
                            xv: 250.0,
                            yv: 250.0,
                            color: C!(WHITE),
                        };
                        game_state = GameState::NotDed;
                    }
                }
                _ => {}
            }

        }
        canvas.present();
        thread::sleep(Duration::from_millis(10));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

    }
}

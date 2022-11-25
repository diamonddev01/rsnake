use piston_window::*;
use rand::Rng;

use crate::colors;
use crate::drawing::{draw_block, draw_text};
use crate::physics::{Direction, Position};
use crate::snake::Snake;
//const RESTART_TIME: f64 = 1.0;

const EASY_FPS: f64 = 10.0;
const MED_FPS: f64 = 15.0;
const HARD_FPS: f64 = 20.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();

    Position {
        x: rng.gen_range(0..width as i32),
        y: rng.gen_range(0..height as i32),
    }
}

pub struct Game {
    snake: Snake,
    fruit: Position,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    over: bool,
    paused: bool,
    fps: f64
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        // use fn defined at eof to calc random fruit / snake pos here
        Self {
            snake: Snake::new(calc_random_pos(width, height)),
            fruit: calc_random_pos(width, height),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            over: false,
            paused: true,
            fps: EASY_FPS
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn toggle_game_state(&mut self) {
        if self.paused {
            self.start();
        } else {
            self.pause();
        }
    }

    pub fn draw(&self, ctx: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        draw_block(&ctx, g, colors::FRUIT, &self.fruit);
        self.snake.draw(&ctx, g);

        let mut owned_string = "".to_owned();
        let string_1 = &self.get_score().to_string();
        let string_2 = " | Running at ";
        let __string_3 = (self.fps / 10 as f64).to_string();
        let string_3 = __string_3.as_str();
        let string_4 = "x speed";

        owned_string.push_str(string_1);
        owned_string.push_str(string_2);
        owned_string.push_str(string_3);
        owned_string.push_str(string_4);

        draw_text(
            &ctx,
            g,
            glyphs,
            colors::SCORE,
            Position { x: 0, y: 20 },
            &owned_string
        );

        if self.over {
            crate::drawing::draw_overlay(&ctx, g, colors::OVERLAY, self.size)
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // if self.over {
        // if self.waiting_time > RESTART_TIME {
        //     self.restart();
        // }
        // return;
        // }

        if self.waiting_time > fps_in_ms(self.fps) && !self.over && !self.paused {
            // self.check_colision() use snake.get_head_pos;
            self.waiting_time = 0.0;

            /*
            pub fn isInAWall(&self) -> bool {
        let next_pos = self.next_head_pos();
        
        // Check snake is in border
        next_pos.x > 0 && next_pos.y > 0 && next_pos.x < self
    }
            */

            if !self.snake.is_tail_overlapping() && !self.snake.will_tail_overlapp() && self.is_in_a_wall() {
                self.snake.update();

                if *self.snake.get_head_pos() == self.fruit {
                    self.snake.grow();
                    self.snake.update();
                    self.fruit = calc_random_pos(self.size.0, self.size.1);
                    self.calc_score();
                }
            } else {
                self.over = true;
            }
        }
    }

    fn is_in_a_wall(&self) -> bool {
        let next_pos = self.snake.next_head_pos();
        let x = next_pos.x;
        let y = next_pos.y;

        x >= 0 && y >= 0 && x <= self.size.0 as i32 -1 && y <= self.size.1 as i32 -1
    }

    pub fn key_down(&mut self, key: keyboard::Key) {
        if self.over && key != Key::R {
            return;
        }
        match key {
            Key::R => self.restart(),
            Key::P | Key::Space => self.toggle_game_state(),
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            _ => {}
        }
    }

    pub fn change_difficulty(&mut self, new_difficulty: u32) {
        match new_difficulty {
            1 => self.fps = EASY_FPS,
            2 => self.fps = MED_FPS,
            3 => self.fps = HARD_FPS,
            v => self.fps = 5 as f64 * v as f64
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn calc_score(&mut self) {
        self.score = (self.snake.get_len()) as u32;
        self.fps = self.score as f64 / 5.0 + 10.0;
    }

    fn restart(&mut self) {
        *self = Game::new(self.size.0, self.size.1);
        self.start();
    }

    // IMPORTANT!! -

    // fn update_snake(&mut self, dir: Option<Direction>) {
    //     if self.check_if_snake_alive(dir) {
    //         self.snake.move_forward(dir);
    //         self.check_eating();
    //     } else {
    //         self.game_over = true;
    //     }
    //     self.waiting_time = 0.0;
    // }
}

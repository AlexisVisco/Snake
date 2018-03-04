use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};
use snake::*;
use draw::*;

const APPLE_COLOR : Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR : Color = [0.0, 0.0, 0.0, 1.0];
const GAME_OVER : Color = [0.9, 0.0, 0.0, 0.5];

const RESTART_TIME : f64 = 1.0;

pub struct Game {
	snake: Snake,
	
	food_exist: bool,
	food_x: i32,
	food_y: i32,
	
	width: i32,
	height: i32,
	
	game_over: bool,
	waiting_time: f64,
	speed: f64,
}

impl Game {
	pub fn new(width: i32, height: i32) -> Game {
		Game {
			snake: Snake::new(2, 2),
			food_exist: true,
			food_x: 6,
			food_y: 4,
			width,
			height,
			game_over: false,
			waiting_time: 0.0,
			speed: 0.4
		}
	}
	
	pub fn key_press(&mut self, key: Key) {
		if self.game_over { return }
		let dir : Option<Direction> = Direction::direction_from_key(key);
		if let Some(direc) = dir {
			if direc == self.snake.head_direction().opposite() { return }
		}
		self.update_snake(dir);
	}
	
	pub fn draw(&self, con: &Context, g2d: &mut G2d) {
		self.snake.draw(con, g2d);
		
		if self.food_exist { draw_block(APPLE_COLOR, self.food_x, self.food_y, con, g2d); }
		
		draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g2d);
		draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g2d);
		draw_rectangle(BORDER_COLOR, 0,  0, 1, self.height, con, g2d);
		draw_rectangle(BORDER_COLOR, self.width - 1,  0, 1, self.height, con, g2d);
		
		if self.game_over { draw_rectangle(GAME_OVER, 0, 0, self.width, self.height, con, g2d); }
	}
	
	pub fn update(&mut self, time: f64) {
		self.waiting_time += time;
		
		if self.game_over {
			if self.waiting_time > RESTART_TIME {
				self.restart();
			}
			return;
		}
		
		if !self.food_exist { self.add_food() }
		println!("speed: {}", self.speed);
		if self.waiting_time > self.speed { self.update_snake(None)}
	}
	
	fn check_eating(&mut self) {
		let (x, y) = self.snake.head_position();
		if self.food_exist && self.food_x == x && self.food_y == y {
			self.food_exist = false;
			self.snake.restore_tail();
		}
	}
	
	fn check_if_snake_alive(&mut self, direction: Option<Direction>) -> bool {
		let (x, y) = self.snake.next_head(direction);
		
		if self.snake.overlap_tail(x, y) { return false }
		x > 0  && y > 0 && x < self.width - 1 && y < self.height - 1
	}
	
	fn add_food(&mut self) {
		let mut rand = thread_rng();
		
		let x = rand.gen_range(1, self.width - 1);
		let y = rand.gen_range(1, self.width - 1);
		
		if self.snake.overlap_tail(x, y) { self.add_food() }
		self.food_exist = true;
		self.food_y = y;
		self.food_x = x;
		self.speed -= 0.025;
		if self.speed < 0.05 { self.speed = 0.05 }
	}
	
	fn update_snake(&mut self, direction: Option<Direction>) {
		if self.check_if_snake_alive(direction) {
			self.snake.move_forward(direction);
			self.check_eating();
		}
		else { self.game_over = true; }
		self.waiting_time = 0.0;
	}
	
	fn restart(&mut self) {
		self.snake = Snake::new(2, 2);
		self.waiting_time = 0.0;
		self.add_food();
		self.game_over = false;
		self.speed = 0.4;
	}
}
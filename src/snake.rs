use std::collections::{LinkedList};
use piston_window::{G2d, Context, Key};
use piston_window::types::Color;

use draw::draw_block;

const SNAKE_COLOR : Color = [0.0, 0.8, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
	Up,
	Down,
	Right,
	Left
}

impl Direction {
	pub fn opposite(&self) -> Direction {
		match *self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
		}
	}
	
	pub fn plus(&self) -> (i32, i32) {
		match *self {
			Direction::Up => (0, -1),
			Direction::Down => (0, 1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		}
	}
	
	pub fn plus_tuple(&self, coords: (i32, i32)) -> (i32, i32) {
		match *self {
			Direction::Up =>    (coords.0 +  0   , coords.1 + (-1)),
			Direction::Down =>  (coords.0 +  0   , coords.1 + 1),
			Direction::Left =>  (coords.0 + (-1) , coords.1 + 0),
			Direction::Right => (coords.0 +  1   , coords.1 + 0),
		}
	}
	
	fn block_from_pos(&self, coords: (i32, i32)) -> Block {
		let to_add = self.plus();
		Block {x: coords.0 + to_add.0, y: coords.1 + to_add.1}
	}
	
	pub fn direction_from_key(key: Key) -> Option<Direction> {
		match key {
			Key::Up => Some(Direction::Up),
			Key::Down => Some(Direction::Down),
			Key::Left => Some(Direction::Left),
			Key::Right => Some(Direction::Right),
			_ => None
		}
	}
}

#[derive(Debug, Clone)]
struct Block {
	x: i32,
	y: i32
}

pub struct Snake {
	direction: Direction,
	body: LinkedList<Block>,
	tail: Option<Block>
}

impl Snake {
	pub fn new(x: i32, y: i32) -> Snake {
		let mut body : LinkedList<Block> = LinkedList::new();
		for i in 0 .. 2 {
			body.push_back(Block { x: x + i, y});
		}
		
		Snake {
			direction: Direction::Right,
			body,
			tail: None
		}
	}
	
	pub fn draw(&self, con: &Context, g2d: &mut G2d) {
		for block in &self.body {
			draw_block(SNAKE_COLOR, block.x, block.y, con, g2d);
		}
	}
	
	pub fn head_position(&self) -> (i32, i32) {
		let front = self.body.front().unwrap();
		(front.x, front.y)
	}
	
	pub fn move_forward(&mut self, direction: Option<Direction>) {
		if let Some(d) = direction {
			self.direction = d;
		}
		
		let new_block = self.direction.block_from_pos(self.head_position());
		self.body.push_front(new_block);
		let last = self.body.pop_back().unwrap();
		self.tail = Some(last);
	}
	
	pub fn head_direction(&self) -> Direction {
		self.direction
	}
	
	pub fn next_head(&mut self, mut direction: Option<Direction>) -> (i32, i32) {
		let head_pos = self.head_position();
		let dir = direction.get_or_insert(self.direction);
		self.direction = *dir;
		dir.plus_tuple(head_pos)
	}
	
	pub fn restore_tail(&mut self) {
		let tail = self.tail.clone().unwrap();
		self.body.push_back(tail);
	}
	
	pub fn overlap_tail(&mut self, x: i32, y: i32) -> bool {
		let mut ch = 0;
		for block in &self.body {
			if x == block.x && y == block.y { return true }
			ch += 1;
			if ch == self.body.len() - 1 { break; }
		}
		false
	}
}
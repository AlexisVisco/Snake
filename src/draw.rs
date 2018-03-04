use piston_window::{rectangle, G2d, Context};
use piston_window::types::Color;

const BLOCK_SIZE : f64 = 25.0;

pub fn to_coord(coord: i32) -> f64 {
	BLOCK_SIZE * (coord as f64)
}

pub fn to_coord_u32(coord: i32) -> u32 {
	to_coord(coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g2d: &mut G2d) {
	let mx = to_coord(x);
	let my = to_coord(y);
	
	rectangle(color, [mx, my, BLOCK_SIZE, BLOCK_SIZE], con.transform, g2d);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, w: i32, h: i32, con: &Context, g2d: &mut G2d) {
	let mx = to_coord(x);
	let my = to_coord(y);
	
	rectangle(color, [mx, my, BLOCK_SIZE * (w as f64), BLOCK_SIZE * (h as f64)], con.transform, g2d);
}
use std::ops::{Add};

use rand::Rng;
use sdl2::{sys::INT32_MAX, video::Window};



const SNAKE_MAXLEN: usize = 100;


#[derive(Debug)]
pub struct Point {
	pub x: i32,
	pub y: i32
}

impl Copy for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl Clone for Point {
    fn clone(&self) -> Self {
		Self { x: self.x.clone(), y: self.y.clone() }
	}
}

impl Add for Point {
    type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: rhs.x + self.x,
			y: rhs.y + self.y,
		}
	}
}


impl Point {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			x,
			y,
		}
	}
}


pub struct SnakeLogic {
	head: usize,

	snake: [Point; SNAKE_MAXLEN],

	snake_len: i32,
	
	apple: Point,
	
	box_width: u32,
	
	width: u32,
	height: u32,
}


impl SnakeLogic {
	pub fn render(&self, canvas: &mut sdl2::render::Canvas<Window>, offset: Point) {
		let mut rect = sdl2::rect::Rect::new(0, 0, self.box_width, self.box_width);
		
		canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));

		
		rect.x = self.apple.x * self.box_width as i32 + offset.x;
		rect.y = self.apple.y * self.box_width as i32 + offset.y;

		canvas.fill_rect(rect);
		
				
		for i in 0..self.snake_len as usize {
			let mut v: usize = i + self.head;
			
			v -= if v >= SNAKE_MAXLEN { SNAKE_MAXLEN } else { 0 };
			
			if v == self.head {
				canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
			} else {
				canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
			}
	
			rect.x = self.snake[v].x * self.box_width as i32;
			rect.y = self.snake[v].y * self.box_width as i32;

			canvas.fill_rect(rect);
		}
		
		canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
		
		for x in (0..self.width as i32).step_by(self.box_width as usize) {
			for i in 0..7 as i32 {
				canvas.draw_line(sdl2::rect::Point::new(x + i, 0), sdl2::rect::Point::new(x + i, self.height as i32));
			}
		}
		
		for y in (0..self.height as i32).step_by(self.box_width as usize) {
			for i in 0..7 as i32 {
				canvas.draw_line(sdl2::rect::Point::new(0, y + i), sdl2::rect::Point::new(self.width as i32, y + i));
			}
		}

		canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
	}
	
	pub fn apple_regen(&mut self) {
		loop {
			self.apple.x = (rand::thread_rng().gen_range(0..INT32_MAX) % (self.width / self.box_width)) as i32;
			self.apple.y = (rand::thread_rng().gen_range(0..INT32_MAX) % (self.height / self.box_width)) as i32;

			for i in 0..self.snake_len as usize{
				let mut v: usize = i + self.head;
				
				v -= if v >= SNAKE_MAXLEN { SNAKE_MAXLEN } else { 0 };
				
				if self.snake[v] == self.apple {
					continue;
				}
			}

			break;
		}
	}
	
	pub fn update(&mut self, dir: Point) {
		if self.snake_len == 0 {
			self.snake_len = 5;
			self.head = 0;

			for i in 0..self.snake_len {
				self.snake[(self.snake_len - i - 1) as usize].x = i + 1;
				self.snake[(self.snake_len - i - 1) as usize].y = 2;
			}
			
			self.apple_regen();
		} else {
			let n_head = if self.head > 0 { self.head - 1 } else { SNAKE_MAXLEN - 1 };
			self.snake[n_head] = self.snake[self.head] + dir;
			self.head = n_head;

			if self.snake[n_head].x > (self.width / self.box_width) as i32 - 1 {
				self.snake[n_head].x = 0;
			}

			if self.snake[n_head].y > (self.height / self.box_width) as i32  - 1 {
				self.snake[n_head].y = 0;
			}


			if self.snake[n_head].x < 0 {
				self.snake[n_head].x = (self.width / self.box_width) as i32 - 1;
			}

			if self.snake[n_head].y < 0 {
				self.snake[n_head].y = (self.height / self.box_width) as i32  - 1;
			}

			for i in 1..self.snake_len as usize {
				let mut v: usize = i + self.head;
				
				v -= if v >= SNAKE_MAXLEN { SNAKE_MAXLEN } else { 0 };
				
				if self.snake[v] == self.snake[self.head] {
					println!("Game Over!");
					
					self.snake_len = 0;

					return;
				}
			}
			
			if self.apple == self.snake[self.head] {
				println!("points: {}", self.snake_len - 5);
				
				self.snake_len += 1;
				
				self.apple_regen();
			}
		}
	}

	pub fn new(width: u32, height: u32, box_width: u32) -> Self {
		Self {
			head: 0,
			
			snake: [Point { x: 0, y: 0 }; SNAKE_MAXLEN],
			snake_len: 0,
			
			apple: Point { x: 0, y: 0 },
			
			box_width,

			width,
			height,
		}
	}
}
use std::ops::{Add, Index};



const SNAKE_MAXLEN: usize = 100;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
	x: i32,
	y: i32
}

impl Add for Point {
    type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: rhs.x + self.x,
			y: rhs.x + self.x,
		}
	}
}


impl Point {
	pub fn init(x: i32, y: i32) -> Self {
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
	
	snake_dir: Point
}


impl SnakeLogic {
	pub fn apple_regen(&mut self) {
		loop {
			self.apple.x = rand::random::<i32>() % (self.width / self.box_width) as i32;
			self.apple.y = rand::random::<i32>() % (self.height / self.box_width) as i32;

			for el in self.snake {
				if el == self.apple {
					continue;
				}
			}

			return;
		}
	}
	
	pub fn update(&mut self, mut dir: Point) {
		if self.snake_len == 0 {
			self.snake_len = 5;

			for i in 0..self.snake_len {
				self.snake[(5 - i - 1) as usize].x = i + 1;
				self.snake[(5 - i - 1) as usize].y = 2;
			}
			
			self.apple_regen();
		} else {
			for i in 0..2 {
				let mut new_head_p = self.snake[self.head] + dir;
				
				
				if new_head_p.x > (self.width / self.box_width) as i32 - 1 {
					new_head_p.x = 0;
				}
	
				if new_head_p.y > (self.height / self.box_width) as i32  - 1 {
					new_head_p.y = 0;
				}
	
	
				if new_head_p.x < 0 {
					new_head_p.x = (self.width / self.box_width) as i32 - 1;
				}
	
				if new_head_p.y < 0 {
					new_head_p.y = (self.height / self.box_width) as i32  - 1;
				}
				
				
				if new_head_p.x != self.snake[self.head + 1].x || new_head_p.y != self.snake[self.head + 1].y {
					self.head = if self.head > 0 { self.head - 1 } else { SNAKE_MAXLEN - 1 };
					
					self.snake[self.head] = new_head_p;
					self.snake_dir = dir;
				} else {
					dir = self.snake_dir;
				}
			}
				
			for i in 0..self.snake_len as usize {
				let mut v: usize = i + self.head;
				
				v -= if v >= SNAKE_MAXLEN { SNAKE_MAXLEN } else { 0 };
				
				if self.snake[v] == self.snake[self.head] {
					
				}
			}
				
				// for (int i = 1; i < snake_len; i++) {
				// 	int v = i + head;

				// 	v -= (v >= SNAKE_BUFFER_LIMIT) ? SNAKE_BUFFER_LIMIT: 0;
					
				// 	if (snake[v].x == snake[head].x && snake[v].y == snake[head].y) {
				// 		printf("game over!\n");
						
				// 		snake_len = 0;
						
				// 		break;
				// 	}
				// }


				// if (apple.x == snake[head].x && apple.y == snake[head].y) {
				// 	snake_len += 1;
					
				// 	apple_regen();
					
				// 	printf("points: %d\n", snake_len - 5);
				// }
		}
	}

	pub fn init(width: u32, height: u32, box_width: u32) -> Self {
		Self {
			head: 0,
			
			snake: [Point { x: 0, y: 0 }; SNAKE_MAXLEN],
			snake_len: 0,
			
			apple: Point { x: 0, y: 0 },
			
			box_width,

			width,
			height,
			
			snake_dir: Point { x: 1, y: 0 },
		}
	}
}
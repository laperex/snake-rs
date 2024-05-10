mod rnake;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {
	const width: u32 = 1920;
	const height: u32 = 1080;
	const box_width: u32 = 40;
	
    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem.window("rust-sdl2 demo", 1920, 1080)
    //     .position_centered()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();

    // let mut event_pump = sdl_context.event_pump().unwrap();

	let mut snake = rnake::SnakeLogic::init(width, height, box_width);

	snake.update(rnake::Point::init(1, 0));

    // 'running: loop {
    //     canvas.clear();

    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
    //                 break 'running
    //             },
    //             _ => {}
    //         }
    //     }
    //     // The rest of the game loop goes here...
		
		

    //     canvas.present();

    //     std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }
}
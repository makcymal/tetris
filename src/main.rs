#![allow(warnings)]

mod color;
mod sdl;
mod tetris;

use {
    sdl::Sdl,
    tetris::Tetris,
    sdl2::{
    	event::Event,
		keyboard::Keycode,
    },
    std::{
    	time::{
    		Duration,
    		Instant,
    	},
    	thread,
    },
};


fn main() {
    let mut tetris = Tetris::new();
    let mut sdl = Sdl::init();

    // tetris proceeding, regard to the current level
    let mut time_to_proceed = true;
    let mut last_proceeded = Instant::now();
    
    // the inverse of fps
    let delay = Duration::from_millis(1000 / 50);
    // used in order to make duration between rendering equal
    let mut postponed_delay = Duration::ZERO;


    'runtime: loop {
    	// descent or kill or generate tetrimino
    	if time_to_proceed {
    		tetris.proceed();

    		time_to_proceed = false;
    		last_proceeded = Instant::now();

    		// this delay was carried from the prev sleep
    		if !postponed_delay.is_zero() {
    			thread::sleep(postponed_delay);
    			postponed_delay = Duration::ZERO;
			}
    	}

    	// rendering


        // handling events
        for event in sdl.listen() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'runtime;
                }
                // other events don't relates to the runtime
                _ => tetris.react_to(event),
            };
        }

        // if it necessary to proceed tetris not after the full delay
        if tetris.level_time() <= last_proceeded.elapsed() + delay {
        	// curr_delay >= 0
        	let curr_delay = tetris.level_time()
        		.saturating_sub(last_proceeded.elapsed());

        	thread::sleep(curr_delay);
        	// this delay will be performed after proceeding tetris
        	postponed_delay = delay - curr_delay;
        	time_to_proceed = true;
        } else {
        	thread::sleep(delay);
        }
    }
}

extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use modulo::Mod;

use std::time::Duration;

const CELLMAP_WIDTH: i32 = 400;
const CELLMAP_HEIGHT: i32 = 400;
const CELLMAP_SIZE: i32 = CELLMAP_WIDTH * CELLMAP_HEIGHT;

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 1200;
const SCREEN_SIZE: i32 = SCREEN_WIDTH * SCREEN_HEIGHT;

const CELL_WIDTH: i32 = SCREEN_WIDTH / CELLMAP_WIDTH;
const CELL_HEIGHT: i32 = SCREEN_HEIGHT / CELLMAP_HEIGHT;

const FPS : i32 = 60;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
const BLUE: Color = Color::RGB(0, 0, 255);

const DEAD: u8 = 0;
const ALIVE: u8 = 1;
const DYING: u8 = 2;

pub fn random_cellmap() -> [u8; CELLMAP_SIZE as usize] {
    let mut cellmap: [u8; CELLMAP_SIZE as usize] = [0; CELLMAP_SIZE as usize];
    for i in 0..CELLMAP_SIZE-1 {
        cellmap[i as usize] = rand::random::<bool>() as u8;
    }
    return cellmap;
}

pub fn line() -> [u8; CELLMAP_SIZE as usize] {
    let mut cellmap: [u8; CELLMAP_SIZE as usize] = [0; CELLMAP_SIZE as usize];
    for i in 0..CELLMAP_SIZE-1 {
        if i % CELLMAP_WIDTH == 0 {
            cellmap[i as usize] = ALIVE;
        }
    }
    return cellmap;
}

pub fn GoL(prev_gen: [u8; CELLMAP_SIZE as usize]) -> [u8; CELLMAP_SIZE as usize] {

    let mut next_gen: [u8; CELLMAP_SIZE as usize] = [DEAD; CELLMAP_SIZE as usize];

    for i in 0..CELLMAP_SIZE-1 {

        let cell = prev_gen[i as usize];
        let mut col: i32 = -1;
        let mut row: i32 = -1;
        
        let mut neighbors: i32 = 0;
        let pos = i as i32;

        let neighbors_pos: [i32; 8] = [pos - 1, pos + 1, pos - CELLMAP_WIDTH, pos + CELLMAP_WIDTH, pos - CELLMAP_WIDTH - 1, pos - CELLMAP_WIDTH + 1, pos + CELLMAP_WIDTH - 1, pos + CELLMAP_WIDTH + 1];
        for p in neighbors_pos {
            if prev_gen[(p.modulo(CELLMAP_SIZE)) as usize] == ALIVE {
                neighbors += 1;
            }
        }

        if cell == ALIVE {
            if neighbors < 2 || neighbors > 3 {
                next_gen[i as usize] = DEAD;
            } else {
                next_gen[i as usize] = ALIVE;
            }
        } else if neighbors == 3 {
            next_gen[i as usize] = ALIVE;
        }
    }
    return next_gen;
 }

 pub fn Brain(prev_gen: [u8; CELLMAP_SIZE as usize]) -> [u8; CELLMAP_SIZE as usize] {

    let mut next_gen: [u8; CELLMAP_SIZE as usize] = [DEAD; CELLMAP_SIZE as usize];

    for i in 0..CELLMAP_SIZE-1 {

        let cell = prev_gen[i as usize];
        let mut col: i32 = -1;
        let mut row: i32 = -1;
        
        let mut neighbors: i32 = 0;
        let pos = i as i32;

        let neighbors_pos: [i32; 8] = [pos - 1, pos + 1, pos - CELLMAP_WIDTH, pos + CELLMAP_WIDTH, pos - CELLMAP_WIDTH - 1, pos - CELLMAP_WIDTH + 1, pos + CELLMAP_WIDTH - 1, pos + CELLMAP_WIDTH + 1];
        for p in neighbors_pos {
            if prev_gen[(p.modulo(CELLMAP_SIZE)) as usize] == ALIVE {
                neighbors += 1;
            }
        }

        if cell == ALIVE {
            next_gen[i as usize] = DYING
        } else if cell == DYING {
            next_gen[i as usize] = DEAD
        } else if neighbors == 2 {
            next_gen[i as usize] = ALIVE
        }
    }
    return next_gen;

 }


pub fn main() {

    let mut cellmap = random_cellmap();

    let sdl_context = sdl2::init().unwrap();
    let timer = sdl_context.timer().unwrap();
    let frame_delay = 1000 / FPS as i32;
    let ticks = timer.ticks() as i32;

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust Life", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();

    canvas.present();

    let mut paused = true;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.set_draw_color(WHITE);
        for i in 0..CELLMAP_SIZE {
            let col = i as i32 / CELLMAP_WIDTH;
            let row = i as i32 % CELLMAP_WIDTH;
            if cellmap[i as usize] == ALIVE {
                canvas.set_draw_color(WHITE);
                canvas.fill_rect(Rect::new((col * CELL_WIDTH) as i32, (row * CELL_HEIGHT) as i32, CELL_WIDTH as u32, CELL_HEIGHT as u32)).unwrap();
            } else if cellmap[i as usize] == DYING {
                canvas.set_draw_color(BLUE);
                canvas.fill_rect(Rect::new((col * CELL_WIDTH) as i32, (row * CELL_HEIGHT) as i32, CELL_WIDTH as u32, CELL_HEIGHT as u32)).unwrap();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    cellmap = random_cellmap();
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;
                },
                _ => {}
            }
        }
        if !paused {
            cellmap = GoL(cellmap);
            //cellmap = Brain(cellmap);
        }

        canvas.present();

        //std::thread::sleep(Duration::from_millis(frame_delay as u64));
    }
}
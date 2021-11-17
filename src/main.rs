extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::mouse::MouseButton;

use modulo::Mod;

const CELLMAP_WIDTH: i32 = 200;
const CELLMAP_HEIGHT: i32 = 200;
const CELLMAP_SIZE: i32 = CELLMAP_WIDTH * CELLMAP_HEIGHT;

const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 1200;

const CELL_WIDTH: i32 = SCREEN_WIDTH / CELLMAP_WIDTH;
const CELL_HEIGHT: i32 = SCREEN_HEIGHT / CELLMAP_HEIGHT;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
const BLUE: Color = Color::RGB(0, 0, 255);

const DEAD: u8 = 0;
const ALIVE: u8 = 1;
const DYING: u8 = 2;

fn random_cellmap() -> [u8; CELLMAP_SIZE as usize] {
    let mut cellmap: [u8; CELLMAP_SIZE as usize] = [0; CELLMAP_SIZE as usize];
    for i in 0..CELLMAP_SIZE-1 {
        cellmap[i as usize] = rand::random::<bool>() as u8;
    }
    return cellmap;
}

fn new_cellmap() -> [u8; CELLMAP_SIZE as usize] {
    let mut cellmap: [u8; CELLMAP_SIZE as usize] = [0; CELLMAP_SIZE as usize];
    for i in 0..CELLMAP_SIZE-1 {
        cellmap[i as usize] = DEAD;
    }
    return cellmap;
}

fn life(cellmap: [u8; CELLMAP_SIZE as usize]) -> [u8; CELLMAP_SIZE as usize] {

    let mut next_gen: [u8; CELLMAP_SIZE as usize] = [DEAD; CELLMAP_SIZE as usize];

    for i in 0..CELLMAP_SIZE {
        
        let neighbors: u8 = cellmap[(i - 1 - CELLMAP_WIDTH).modulo(CELLMAP_SIZE) as usize]+
            cellmap[(i - CELLMAP_WIDTH).modulo(CELLMAP_SIZE) as usize]+
            cellmap[(i + 1 - CELLMAP_WIDTH).modulo(CELLMAP_SIZE) as usize]+
            cellmap[(i - 1).modulo(CELLMAP_SIZE) as usize]+
            cellmap[(i + 1).modulo(CELLMAP_SIZE) as usize]+
            cellmap[(i - 1 + CELLMAP_WIDTH).modulo(CELLMAP_SIZE) as usize]+
            cellmap[(i + CELLMAP_WIDTH).modulo(CELLMAP_SIZE) as usize]+
            cellmap[(i + 1 + CELLMAP_WIDTH).modulo(CELLMAP_SIZE) as usize];

        if cellmap[i as usize] == ALIVE {
            if neighbors < 2 || neighbors > 3 {
                next_gen[i as usize] = DEAD;
                continue;
            } else{
                next_gen[i as usize] = ALIVE;
                continue;
            }
        } else if neighbors == 3 {
            next_gen[i as usize] = ALIVE;
            continue;
        }
    }
    return next_gen;
}

fn sim(num_generations: i32, start_paused: bool) {

    let mut cellmap = random_cellmap();

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust Life", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();

    canvas.present();

    let mut mouse_click = false;
    let mut mouse_hold = false;
    let mut hold_value = DEAD;

    let mut last_mouse_pos = (0, 0);

    let mut paused = start_paused;
    let mut generation = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {

        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.set_draw_color(WHITE);
        for i in 0..CELLMAP_SIZE {
            let col: i32 = i as i32 / CELLMAP_WIDTH;
            let row: i32 = (i as i32).modulo(CELLMAP_WIDTH);
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
                Event::Quit {..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    cellmap = new_cellmap();
                    generation = 0;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    cellmap = random_cellmap();
                    generation = 0;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;
                },
                Event::KeyDown { keycode: Some(Keycode::Period), .. } => {
                    cellmap = life(cellmap);
                    generation += 1;
                },

                Event::MouseMotion { x, y, .. } => {
                    let col: i32 = x / CELL_WIDTH;
                    let row: i32 = y / CELL_HEIGHT;
                    last_mouse_pos = (col, row);
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                    mouse_click = true;
                    mouse_hold = true;
                },
                Event::MouseButtonUp { x, y, mouse_btn: MouseButton::Left, .. } => {
                    mouse_hold = false;
                },
                _ => {}
            }
        }

        if mouse_click {
            let col: i32 = last_mouse_pos.0;
            let row: i32 = last_mouse_pos.1;
            let i: i32 = (col * CELLMAP_WIDTH + row) as i32;
            if cellmap[i as usize] == DEAD {
                cellmap[i as usize] = ALIVE;
                hold_value = ALIVE;
            } else {
                cellmap[i as usize] = DEAD;
                hold_value = DEAD;
            }
            mouse_click = false;
        }
        if mouse_hold {
            if hold_value == ALIVE {
                cellmap[(last_mouse_pos.0 * CELLMAP_WIDTH + last_mouse_pos.1) as usize] = ALIVE;
            } else { 
                cellmap[(last_mouse_pos.0 * CELLMAP_WIDTH + last_mouse_pos.1) as usize] = DEAD;
            }
        }

        if !paused {
            cellmap = life(cellmap);
            generation += 1;
        }

        if generation == num_generations {
            break 'running
        }

        canvas.present();
    }
}

fn main() {
    sim(-1, false);
}
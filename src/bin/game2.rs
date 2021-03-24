use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use rand::Rng;

use Unit2_2D::{collision::*, health::*, screen::Screen, sprite::*, texture::Texture, tiles::*, types::*};

type Level = (Tilemap, Vec<Sprite>);

struct GameState {
    player: Sprite,
    // TODO: Add in a way to keep track of the enemies for each level
    // Change this maybe? Hearts vs health bar
    health: HealthStatus,
    contacts: Vec<Contact>,
    window: Vec2i,
    level: usize
    // TODO: Add in game state when we have implementation from game 1
    // TODO: Create a way to display the fighting mode
}

const WIDTH: usize = 320;
const HEIGHT: usize = 256;
const DEPTH: usize = 4;
const DT: f64 = 1.0 / 60.0;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Stop the Spread")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    // TODO: Once we find the texture we want to use replace this path and delete the current placeholder file
    let tex = Rc::new(Texture::with_file(Path::new("content/bob.png")));
    let level1_tex = Rc::new(Texture::with_file(Path::new("content/level1.png")));
    let level2_tex = Rc::new(Texture::with_file(Path::new("content/level2.png")));
    let level3_tex = Rc::new(Texture::with_file(Path::new("content/level3.png")));
    let health_tex = Rc::new(Texture::with_file(Path::new("content/Heart.png")));
    let inside_tex = Rc::new(Texture::with_file(Path::new("content/inside.png")));
    let tileset1 = Rc::new(Tileset::new(
        {
            (0..75)
            .map(|i| (
                if i == 11 || i == 20 || i == 10 || i == 13 || i == 71 || i == 70 || i == 23 || i == 30 || i == 33 || i == 50 || i == 52 || i == 53 {
                    Tile { solid: true }
                } else {
                    Tile { solid: false }
                }
            ))
            .collect()
        },
        &level1_tex,
    ));
    let tileset2 = Rc::new(Tileset::new(
        {
            (0..75)
            .map(|i| (
                if i == 11 || i == 20 || i == 10 || i == 13 || i == 71 || i == 70 || i == 23 || i == 30 || i == 33 || i == 50 || i == 52 || i == 53 {
                    Tile { solid: true }
                } else {
                    Tile { solid: false }
                }
            ))
            .collect()
        },
        &level2_tex,
    ));

    let tileset3 = Rc::new(Tileset::new(
        {
            (0..75)
            .map(|i| (
                if i == 11 || i == 20 || i == 10 || i == 13 || i == 71 || i == 70 || i == 23 || i == 30 || i == 33 || i == 50 || i == 52 || i == 53 {
                    Tile { solid: true }
                } else {
                    Tile { solid: false }
                }
            ))
            .collect()
        },
        &level3_tex,
    ));

    let levels: Vec<Level> = vec![
        (Tilemap::new(
            Vec2i(0, 0),
            (10, 9),
            &tileset1,
            vec![
                10, 11, 11, 11, 11, 41, 11, 11, 11, 13,
                20, 22, 21, 22, 21, 41, 21, 22, 21, 23,
                30, 31, 32, 32, 32, 32, 32, 32, 32, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                50, 52, 52, 70, 41, 71, 52, 52, 52, 53,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ), vec![Sprite::new(
            &inside_tex,
            Rect {
                x: 352,
                y: 768,
                w: 32,
                h: 64,
            },
            Vec2i(160, 0),
            true,
        )] ),
        (Tilemap::new(
            Vec2i(0, 0),
            (10, 13),
            &tileset2,
            vec![
                10, 11, 11, 11, 11, 41, 11, 11, 11, 13,
                20, 22, 21, 22, 21, 41, 21, 22, 21, 23,
                30, 31, 32, 32, 32, 32, 32, 32, 32, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                50, 52, 52, 70, 41, 71, 52, 52, 52, 53,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ), vec![Sprite::new(
            &inside_tex,
            Rect {
                x: 352,
                y: 768,
                w: 32,
                h: 64,
            },
            Vec2i(160, 0),
            true,
        )] ),
        (Tilemap::new(
            Vec2i(0, 0),
            (10, 13),
            &tileset3,
            vec![
                10, 11, 11, 11, 11, 11, 11, 11, 11, 13,
                20, 22, 21, 22, 21, 22, 21, 22, 21, 23,
                30, 31, 32, 32, 32, 32, 32, 32, 32, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                30, 41, 42, 42, 42, 42, 42, 42, 42, 33,
                50, 52, 52, 70, 41, 71, 52, 52, 52, 53,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 30, 41, 33, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ), vec![Sprite::new(
            &inside_tex,
            Rect {
                x: 352,
                y: 768,
                w: 32,
                h: 64,
            },
            Vec2i(160, 0),
            true,
        )] ),
    ];

    let mut state = GameState {
        player: Sprite::new(
            &tex,
            Rect {
                x: 0,
                y: 8,
                w: 16,
                h: 24,
            },
            Vec2i(128, 224),
            true,
        ),
        health: HealthStatus{
            image: Rc::clone(&health_tex),
            lives: 3,
            frame: Rect {
                x:0,
                y:0,
                w:16,
                h:16
            },
            start: Vec2i(120, 15),
            spacing: 18
        },
        contacts: vec![],
        window: Vec2i(0,0), 
        level: 0,
    };
    // How many frames have we simulated
    let mut frame_count: usize = 0;
    // How many unsimulated frames have we saved up
    let mut available_time = 0.0;
    // Track beginning of play
    let start = Instant::now();
    // Track end of the last frame
    let mut since = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, state.window);
            screen.clear(Rgba(0, 0, 0, 0));

            draw_game(&state, &mut screen, &levels);

            // Flip buffers
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Rendering has used up some time.
            // The renderer "produces" time...
            available_time += since.elapsed().as_secs_f64();
        }
        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window if needed
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }
        // And the simulation "consumes" it
        while available_time >= DT {
            // Eat up one frame worth of time
            available_time -= DT;

            update_game(&mut state, &input, frame_count, &levels);
            
            // Increment the frame counter
            frame_count += 1;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &GameState, screen: &mut Screen, levels: &Vec<Level>) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(80, 80, 80, 255));

    // TODO: Draw tiles
    levels[state.level].0.draw(screen);
    for s in levels[state.level].1.iter() {
        screen.draw_sprite(&s);
    }
    // TODO: Draw Sprites
    screen.draw_sprite(&state.player);

}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize, levels: &Vec<Level>) {
    // Player control goes here

    if input.key_held(VirtualKeyCode::Right) {
        // TODO: Add Accel?
        state.player.position.0 += 2;
        state.player.frame.x = 32;
        // TODO: Maybe Animation?
    } else if input.key_held(VirtualKeyCode::Left) {
        // TODO: Add accel?
        state.player.position.0 -= 2;
        state.player.frame.x = 16;
        // TODO: Maybe Animation?
    } else {
        state.player.frame.x = 0;
    }
    if input.key_held(VirtualKeyCode::Up) {
        // TODO: Add Accel?
        state.player.position.1 -= 2;
        // TODO: Maybe Animation?
    } else if input.key_held(VirtualKeyCode::Down) {
        // TODO: Add accel?
        state.player.position.1 += 2;
        // TODO: Maybe Animation?
    }

    // Detect collisions: See if the player is collided with an obstacle
    state.contacts.clear();
    gather_contacts(&levels[state.level].0, &state.player, &mut state.contacts);

    restitute(&levels[state.level].0, &mut state.player, &mut state.contacts);

    if state.player.position.0 < 192 && state.player.position.0 > 160 && state.player.position.1 < 32 {
        state.level += 1;
        state.player.position = Vec2i(128, 352);
        state.window = Vec2i(0, 128)
    }

    if state.player.position.1 > (state.window.1 + HEIGHT as i32 - 17)  {
        state.window.1 += 2;
        if state.window.1 > HEIGHT as i32*2 {
            state.window.1 = HEIGHT as i32*2;
        }
    }
    if state.player.position.1 < (state.window.1 + 16) {
        state.window.1 -= 2;
        if state.window.1 < 0 {
            state.window.1 = 0;
        }
    }

}

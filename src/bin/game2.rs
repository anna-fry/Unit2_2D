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


struct GameState {
    player: Sprite,
    // TODO: How should we store our enemies/the levels of the game?
    map: Tilemap,
    // Change this maybe? Hearts vs health bar
    health: HealthStatus,
    contacts: Vec<Contact>,
    window: Vec2i,
    // TODO: Add in game state when we have implementation from game 1
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
    let tex = Rc::new(Texture::with_file(Path::new("content/penguin.png")));
    let building_tex = Rc::new(Texture::with_file(Path::new("content/spread.png")));
    let health_tex = Rc::new(Texture::with_file(Path::new("content/Heart.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
            Tile { solid: true },
        ],
        32,
        &building_tex,
    ));
    let mut state = GameState {
        player: Sprite::new(
            &tex,
            Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 16,
            },
            Vec2i(160, 220),
            true,
        ),
        map: Tilemap::new(
            Vec2i(0, 0),
            (10, 24),
            &tileset,
            vec![
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 0, 0, 0, 0, 0,
            ],
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
            start: Vec2i(260, 15),
            spacing: 18
        },
        contacts: vec![],
        window: Vec2i(0,0), 
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

            draw_game(&state, &mut screen);

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

            update_game(&mut state, &input, frame_count);
            
            // Increment the frame counter
            frame_count += 1;
        }
        // Request redraw
        window.request_redraw();
        // When did the last frame end?
        since = Instant::now();
    });
}

fn draw_game(state: &GameState, screen: &mut Screen) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(80, 80, 80, 255));

    // TODO: Draw tiles
    state.map.draw(screen);
    // TODO: Draw Sprites
    screen.draw_sprite(&state.player);

}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
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
        state.player.position.1 += 2;
        // TODO: Maybe Animation?
    } else if input.key_held(VirtualKeyCode::Down) {
        // TODO: Add accel?
        state.player.position.1 -= 2;
        // TODO: Maybe Animation?
    }

    // Detect collisions: See if the player is collided with an obstacle
    state.contacts.clear();
    gather_contacts(&state.map, &state.player, &mut state.contacts);

    restitute(&state.map, &mut state.player, &mut state.contacts);

    if state.player.position.1 > (state.window.0 + WIDTH as i32 - 33) {
        state.window.1 += 2;
    }
    if state.player.position.1 < (state.window.0 + 32) {
        state.window.1 -= 2;
    }
}

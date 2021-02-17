use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod screen;
use crate::screen::Screen;
mod types;
use types::*;
mod sprite;
use crate::sprite::Sprite;
mod texture;
use crate::texture::Texture;

//TODO: Fill out state
// The State needs to keep track of the player...
// Add texture when we decide on the texture we want
struct GameState {
    player: Sprite,
}

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const DEPTH: usize = 4;
const DT: f64 = 1.0 / 60.0;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Runner Game")
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
    let tex = Rc::new(Texture::with_file(Path::new("content/king.png")));
    let mut state = GameState {
        player: Sprite::new(
            &tex,
            Rect {
                x: 0,
                y: 16,
                w: 16,
                h: 16,
            },
            Vec2i(90, 200),
        )
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
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
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
    // TODO: Draw Sprites
}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Player control goes here
    if input.key_held(VirtualKeyCode::Right) {
        // TODO: Move our player to the right
        // TODO: Maybe Animation?
    }
    if input.key_held(VirtualKeyCode::Left) {
        // TODO: Move our player to the left
        // TODO: Maybe Animation?
    }   

    // TODO: Detect collisions: See if the player is collided with an obstacle

    // TODO: Handle collisions: Take damage, speed up, or slow down

    // TODO: Scroll the scene
}
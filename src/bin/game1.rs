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

// Mod without brackets looks for a nearby file.
mod screen;
// Then we can use as usual.  The screen module will have drawing utilities.
use screen::Screen;
// Collision will have our collision bodies and contact types
mod collision;
// Lazy glob imports
use collision::*;
// Texture has our image loading and processing stuff
mod texture;
use texture::Texture;
// Sprite will define our movable sprites
mod sprite;
// Lazy glob import, see the extension trait business later for why
use sprite::*;
// And we'll put our general purpose types like color and geometry here:
mod types;
use types::*;

mod tiles;
use tiles::*;

//TODO: Fill out state
// The State needs to keep track of the player...
// Add texture when we decide on the texture we want
struct GameState {
    player: Sprite,
    obstacles: Vec<Sprite>,
    spawn_timer:usize,
    scroll_speed:usize,
    map: Tilemap,

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
    let tileTex = Rc::new(Texture::with_file(Path::new("content/IceTileset.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: true },
            Tile { solid: true },
        ],
        &tileTex,
    ));
    let mut state = GameState {
        player: Sprite::new(
            &tex,
            Rect {
                x: 0,
                y: 16,
                w: 16,
                h: 16,
            },
            Vec2i(160, 20),
            true
        ),
        obstacles: vec![Sprite::new(&tex, Rect{x:0, y:0, w:16, h:16}, Vec2i(100, 0), false); 10],
        spawn_timer: 0,
        scroll_speed: 1,
        map: Tilemap::new(
            Vec2i(0, 0),
            (10, 7),
            &tileset,
            vec![
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
            ],
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
            update_obstacles(&mut state);
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
    for sprite in state.obstacles.iter(){
        screen.draw_sprite(sprite);
    }
}
/**
 * updates all obstacles on screen:
 *  scrolls up
 *  removes obstacles over top of screen
 *  if new obstacles are needed, adds them
 */
fn update_obstacles(state: &mut GameState){
    let mut rng = rand::thread_rng();
    for sprite in state.obstacles.iter_mut(){
        if sprite.drawable{
            sprite.position.1 -= 1;

            if sprite.position.1<=0{
                sprite.position.0 = rng.gen_range(0, WIDTH as i32 -16);
                sprite.position.1 = HEIGHT as i32 - 16;
                sprite.drawable = false;
            }
        }
    }
    
    if state.spawn_timer ==0{
        let mut flipped =false;
        for sprite in state.obstacles.iter_mut(){
            if !sprite.drawable && !flipped{
                sprite.drawable = true;
                flipped = rng.gen_range(0,5)<3;
                //TODO: make obstacles not spawn Together
                //TODO: make diff types of obstacles spawn
            }
        }
        state.spawn_timer =rng.gen_range(16, 50);
    }
    state.spawn_timer -=1;
}


fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Player control goes here

    if input.key_held(VirtualKeyCode::Right) {
        // TODO: Add Accel?
        state.player.position.0 += 2;
        // TODO: Maybe Animation?
    }
    if input.key_held(VirtualKeyCode::Left) {
        // TODO: Add accel?
        state.player.position.0 -= 2;
        // TODO: Maybe Animation?
    }

    // TODO: Detect collisions: See if the player is collided with an obstacle

    // TODO: Handle collisions: Take damage, speed up, or slow down

    // TODO: Scroll the scene
}

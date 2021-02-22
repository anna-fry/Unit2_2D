use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::num;
use rand::Rng;

use Unit2_2D::{collision::*, screen::Screen, sprite::*, texture::Texture, tiles::*, types::*};

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
    let tileTex = Rc::new(Texture::with_file(Path::new("content/Background.png")));
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
        obstacles: vec![Sprite::new(&tex, Rect{x:0, y:0, w:16, h:16}, Vec2i(100, 100), false),Sprite::new(&tex, Rect{x:0, y:0, w:16, h:16}, Vec2i(20, 100), false),Sprite::new(&tex, Rect{x:0, y:0, w:16, h:16}, Vec2i(50, 100), false),Sprite::new(&tex, Rect{x:0, y:0, w:16, h:16}, Vec2i(75, 100), false),Sprite::new(&tex, Rect{x:0, y:0, w:16, h:16}, Vec2i(100, 100), false)],
        spawn_timer: 0,
        scroll_speed: 1,
        map: Tilemap::new(
            Vec2i(0, 0),
            (10, 10),
            &tileset,
            vec![
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
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
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, Vec2i(0, 0));
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
    let mut expired:Vec<usize> = vec![0];
    for sprite in state.obstacles.iter_mut(){
        if sprite.drawable{
            sprite.position.1 -= 1;

            if sprite.position.1<=0{
                //sprite.position.0 = 40;
                sprite.position.1 = HEIGHT as i32 - 16;
                sprite.drawable = false;
            }
        }
    }
    
    if state.spawn_timer ==0{
        let mut flipped = false;
        for sprite in state.obstacles.iter_mut(){
            if !sprite.drawable && !flipped{
                sprite.drawable = true;
                flipped = true;
            }
        }
        state.spawn_timer =20;
    }
    state.spawn_timer -=1;
}

fn update_tiles(state: &mut GameState){
    state.map.position.1 -= 1;
    if state.map.position.1.abs() >= TILE_SZ as i32 {
        state.map.position.1 = 0;
    }
}


fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    // Player control goes here

    if input.key_held(VirtualKeyCode::Right) {
        // TODO: Add Accel?
        state.player.position.0 += 2;
        // TODO: Maybe Animation?
        state.player.position.0+=1;
    }
    if input.key_held(VirtualKeyCode::Left) {
        // TODO: Add accel?
        state.player.position.0 -= 2;
        // TODO: Maybe Animation?
        state.player.position.0-=1;

    }

    // Scroll the scene
    update_obstacles(state);
    update_tiles(state);


    // TODO: Detect collisions: See if the player is collided with an obstacle

    // TODO: Handle collisions: Take damage, speed up, or slow down

}

use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue::Font;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use std::{time::Instant, rc::Rc, collections::HashMap, hash::Hash, path::Path, fs::File, io::Read};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use Unit2_2D::{
    collision::*, health::*, screen::Screen, sprite::*, texture::Texture, tiles::*, types::*,
};

enum GameMode {
    Title,
    Playing,
    GameOver,
}
//TODO: Fill out state
// The State needs to keep track of the player...
// Add texture when we decide on the texture we want
struct GameState {
    mode: GameMode,
    player: Sprite,
    obstacles: Vec<Sprite>,
    spawn_timer: usize,
    scroll_speed: usize,
    map: Tilemap,
    health: HealthStatus,
    contacts: Vec<Contact>,
    fonts: [Font; 1],
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

    let font = include_bytes!("..\\..\\content\\monogram_font.ttf") as &[u8];
    let fonts = [Font::from_bytes(font, fontdue::FontSettings::default()).unwrap()];
    // Testing path stuff, pls ignore 
    //
    // let font_file = File::open(Path::new("content/monogram_font.ttf")).unwrap();
    // let mut buf = [0; 10304];
    // font_file.read(&mut buf);
    // let font = &buf;
    // let fonts = [Font::from_bytes(font, fontdue::FontSettings::default()).unwrap()];

    // TODO: Once we find the texture we want to use replace this path and delete the current placeholder file
    let tex = Rc::new(Texture::with_file(Path::new("content/penguin.png")));
    let tile_tex = Rc::new(Texture::with_file(Path::new("content/Background.png")));
    let health_tex = Rc::new(Texture::with_file(Path::new("content/Heart.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false },
            Tile { solid: false },
            Tile { solid: true },
            Tile { solid: true },
        ],
        &tile_tex,
    ));
    let mut state = GameState {
        mode: GameMode::Title,
        player: Sprite::new(
            &tex,
            Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 16,
            },
            Vec2i(160, 20),
            true,
        ),
        obstacles: vec![
            Sprite::new(
                &tex,
                Rect {
                    x: 0,
                    y: 0,
                    w: 16,
                    h: 16
                },
                Vec2i(100, 0),
                false
            );
            10
        ],
        spawn_timer: 0,
        scroll_speed: 1,
        map: Tilemap::new(
            Vec2i(0, 0),
            (10, 10),
            &tileset,
            vec![
                2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0,
                0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0,
                0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
            ],
        ),
        health: HealthStatus {
            image: Rc::clone(&health_tex),
            lives: 3,
            frame: Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 16,
            },
            start: Vec2i(260, 15),
            spacing: 18,
        },
        contacts: vec![],
        fonts,
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

    match state.mode {
        GameMode::Title => {
            let rasterized = HashMap::new();
            let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
            layout.reset(&LayoutSettings{x: (WIDTH/2) as f32, y: (HEIGHT/2) as f32, ..LayoutSettings::default()});
            layout.append(&state.fonts, &TextStyle::new("Hello", 50.0, 0));
            screen.draw_text(rasterized, &state.fonts[0], &mut layout, Rgba(255, 255, 255, 255));
        }
        GameMode::Playing => {
            // TODO: Draw tiles
            state.map.draw(screen);
            // TODO: Draw Sprites
            screen.draw_sprite(&state.player);
            for sprite in state.obstacles.iter() {
                screen.draw_sprite(sprite);
            }
            screen.draw_health(&state.health);
        }
        GameMode::GameOver => (),
    }
}
/**
 * updates all obstacles on screen:
 *  scrolls up
 *  removes obstacles over top of screen
 *  if new obstacles are needed, adds them
 */
fn update_obstacles(state: &mut GameState) {
    let mut rng = rand::thread_rng();
    for sprite in state.obstacles.iter_mut() {
        if sprite.drawable {
            sprite.position.1 -= 1;

            if sprite.position.1 <= 0 {
                sprite.position.0 = rng.gen_range(0, WIDTH as i32 - 16);
                sprite.position.1 = HEIGHT as i32 - 16;
                sprite.drawable = false;
            }
        }
    }

    if state.spawn_timer == 0 {
        let mut flipped = false;
        for sprite in state.obstacles.iter_mut() {
            if !sprite.drawable && !flipped {
                sprite.drawable = true;
                flipped = rng.gen_range(0, 5) < 3;
                if rng.gen_bool(0.2) {
                    sprite.frame.x = 16;
                } else {
                    sprite.frame.x = 0;
                }
                //TODO: make obstacles not spawn Together
                //TODO: make diff types of obstacles spawn
            }
        }
        state.spawn_timer = rng.gen_range(16, 50);
    }
    state.spawn_timer -= 1;
}

fn update_tiles(state: &mut GameState) {
    state.map.position.1 -= 1;
    if state.map.position.1.abs() >= TILE_SZ as i32 {
        state.map.position.1 = 0;
    }
}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    match state.mode {
        GameMode::Title => {}
        GameMode::Playing => {
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

            // Make sure player stays at same height
            state.player.position.1 = 20;

            // Scroll the scene
            update_obstacles(state);
            update_tiles(state);

            // Detect collisions: See if the player is collided with an obstacle
            state.contacts.clear();
            gather_contacts(&state.map, &state.player, &mut state.contacts);

            // TODO: Handle collisions: Take damage, speed up, or slow down
            restitute(&state.map, &mut state.player, &mut state.contacts);
        }
        GameMode::GameOver => (),
    }
}

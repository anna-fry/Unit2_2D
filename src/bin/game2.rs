use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    Font,
};
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use std::{fs::read, path::Path, rc::Rc, time::Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use Unit2_2D::{
    collision::*,
    health::*,
    screen::Screen,
    sprite::*,
    texture::Texture,
    tiles::*,
    types::*,
    text::*,
};

type Level = (Tilemap, Vec<Sprite>);

enum GameMode {
    Title,
    Map,
    Fight,
    GameOver,
}

struct GameState {
    mode: GameMode,
    player: Sprite,
    // TODO: Add in a way to keep track of the enemies for each level
    // Change this maybe? Hearts vs health bar
    health: HealthStatus,
    contacts: Vec<Contact>,
    window: Vec2i,
    level: usize,
    // TODO: Add in game state when we have implementation from game 1
    // TODO: Create a way to display the fighting mode
    passed: bool,
    fonts: Fonts,
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
            .with_title("Title")
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

    let font: &[u8] = &read(Path::new("content/monogram_font.ttf")).unwrap();
    let fonts = [Font::from_bytes(font, fontdue::FontSettings::default()).unwrap()];

    // TODO: Once we find the texture we want to use replace this path and delete the current placeholder file
    let tex = Rc::new(Texture::with_file(Path::new("content/dino.png")));
    let enemy_tex = Rc::new(Texture::with_file(Path::new("content/dinor.png")));
    let level_tex = Rc::new(Texture::with_file(Path::new("content/dungeon.png")));
    let health_tex = Rc::new(Texture::with_file(Path::new("content/Heart.png")));
    let inside_tex = Rc::new(Texture::with_file(Path::new("content/inside.png")));
    let tileset = Rc::new(Tileset::new(
        {
            (0..64)
            .map(|i| (
                if i == 0 || i == 2 || i == 1 || i == 30 || i == 16 || i == 17 || i == 18  {
                    Tile { solid: true }
                } else {
                    Tile { solid: false }
                }
            ))
            .collect()
        },
        &level_tex,
    ));

    let levels: Vec<Level> = vec![
        (Tilemap::new(
            Vec2i(0, 0),
            (10, 9),
            &tileset,
            vec![
                0, 1, 1, 1, 1, 19, 1, 1, 1, 2,
                0, 32, 33, 33, 33, 41, 33, 33, 34, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                16, 17, 17, 17, 40, 17, 17, 17, 17, 18,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        ), vec![Sprite::new(
            &level_tex,
            Rect {
                x: 160,
                y: 0,
                w: 32,
                h: 64,
            },
            Vec2i(160, 0),
            true,
            ),
            Sprite::new(
            &enemy_tex,
            Rect {
                x: 0,
                y: 0,
                w: 24,
                h: 24,
            },
            Vec2i(164, 32),
            true,
        )] ),
        (Tilemap::new(
            Vec2i(0, 0),
            (10, 13),
            &tileset,
            vec![
                0, 1, 1, 1, 1, 19, 1, 1, 1, 2,
                0, 32, 33, 33, 33, 41, 33, 33, 34, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                16, 17, 17, 17, 40, 17, 17, 17, 17, 18,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            ],
        ), vec![Sprite::new(
            &level_tex,
            Rect {
                x: 160,
                y: 0,
                w: 32,
                h: 64,
            },
            Vec2i(160, 0),
            true,
            ),
            Sprite::new(
            &enemy_tex,
            Rect {
                x: 0,
                y: 0,
                w: 24,
                h: 24,
            },
            Vec2i(164, 32),
            true,
        )] ),
        (Tilemap::new(
            Vec2i(0, 0),
            (10, 13),
            &tileset,
            vec![
                0, 1, 1, 1, 1, 19, 1, 1, 1, 2,
                0, 32, 33, 33, 33, 41, 33, 33, 34, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                0, 40, 41, 41, 41, 41, 41, 41, 42, 2,
                16, 17, 17, 17, 40, 17, 17, 17, 17, 18,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            ],
        ), vec![Sprite::new(
            &level_tex,
            Rect {
                x: 160,
                y: 0,
                w: 32,
                h: 64,
            },
            Vec2i(160, 0),
            true,
            ),
            Sprite::new(
            &enemy_tex,
            Rect {
                x: 0,
                y: 0,
                w: 24,
                h: 24,
            },
            Vec2i(164, 32),
            true,
        )] ),
        (Tilemap::new(
            Vec2i(0, 0),
            (10, 13),
            &tileset,
            vec![
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 0, 1, 1, 1, 2, 30, 30, 30,
                30, 30, 0, 32, 33, 33, 2, 30, 30, 30,
                30, 30, 0, 40, 41, 41, 2, 30, 30, 30,
                30, 30, 0, 40, 41, 41, 2, 30, 30, 30,
                30, 30, 0, 40, 41, 41, 2, 30, 30, 30,
                30, 30, 16, 17, 40, 17, 18, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 0, 40, 2, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            ],
        ), vec![Sprite::new(
            &level_tex,
            Rect {
                x: 160,
                y: 64,
                w: 32,
                h: 32,
            },
            Vec2i(128, 128),
            true,
            ),
        ] ),
    ];

    let mut state = GameState {
        mode: GameMode::Title,
        player: Sprite::new(
            &tex,
            Rect {
                x: 0,
                y: 0,
                w: 20,
                h: 24,
            },
            Vec2i(136, 224),
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
        passed: false,
        fonts: Fonts::new(fonts),
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

            draw_game(&mut state, &mut screen, &levels);

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

fn draw_game(state: &mut GameState, screen: &mut Screen, levels: &Vec<Level>) {
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(80, 80, 80, 255));

    match state.mode {
        GameMode::Title => {
            // draws menu screen
            // levels[state.level].0.draw(screen);

            let w = WIDTH as i32;
            let h = HEIGHT as i32;
            let menu_rect = Rect{x: w/6, y: h/8, w: (2*w as u16)/3, h: (2*h as u16)/3};

            screen.rect(menu_rect, Rgba(53, 40, 33, 255));
            screen.empty_rect(menu_rect, 4, Rgba(250, 30, 10, 255));

            let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (HEIGHT / 6) as f32,
                max_width: Some(((2*w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(&state.fonts.font_list, &TextStyle::new("DUNGEONS\nand\nDINOS", 45.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (2*HEIGHT / 3) as f32,
                max_width: Some(((2*w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(&state.fonts.font_list, &TextStyle::new("Press ENTER to start", 20.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
        }
        GameMode::Map => {
            levels[state.level].0.draw(screen);
            for s in levels[state.level].1.iter() {
                screen.draw_sprite(&s);
            }
            screen.draw_sprite(&state.player);
        }
        GameMode::Fight => {
            // TODO: Render the fight screen
        }
        GameMode::GameOver => {
            // TODO: Create game over screen
        },
    }

}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize, levels: &Vec<Level>) {

    match state.mode {
        GameMode::Title => {
            if input.key_held(VirtualKeyCode::Return) {
                state.mode = GameMode::Map;
            }
        }
        GameMode::Map => {
            if input.key_held(VirtualKeyCode::Right) {
                // TODO: Add Accel?
                state.player.position.0 += 2;
                // TODO: Maybe Animation?
            } else if input.key_held(VirtualKeyCode::Left) {
                // TODO: Add accel?
                state.player.position.0 -= 2;
                // TODO: Maybe Animation?
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
            gather_contacts(&levels[state.level].0, &state.player, &levels[state.level].1, &mut state.contacts);

            restitute(&levels[state.level].0, &mut state.player, &levels[state.level].1, &mut state.contacts);

            if state.player.position.0 < 192 && state.player.position.0 > 160 && state.player.position.1 < 8 {
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
        GameMode::Fight => {
            // TODO: Read in attack choices and do state logic
        }
        GameMode::GameOver => {
            // TODO: Reset Game
        }
    }
}

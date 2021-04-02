use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    Font,
};
use image::imageops::vertical_gradient;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use std::{fs::read, path::Path, rc::Rc, time::Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use Unit2_2D::{
    animation::*,
    collision::*,
    health::*,
    screen::Screen,
    sprite::*,
    texture::Texture,
    tiles::*,
    types::*,
    text::*,
};

type Level = (Vec<Tilemap>, Vec<Sprite>, usize);

enum GameMode {
    Title,
    Map,
    Fight,
    FightChoice,
    GameOver,
    Win,
}

struct GameState {
    mode: GameMode,
    player: Sprite,
    health: HealthStatus,
    enemy_health: HealthStatus,
    player_choice: Attack,
    enemy_choice: Attack,
    choice_frame: usize,
    contacts: Vec<Contact>,
    window: Vec2i,
    level: usize,
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
    let tileset = Rc::new(Tileset::new(
        {
            (0..64)
            .map(|i| (
                if i == 0 || i == 2 || i == 1 || i == 30 || i == 16 || i == 17 || i == 18  {
                    Tile { solid: true, collide:Effect::Nothing }
                } else {
                    Tile { solid: false, collide:Effect::Nothing }
                }
            ))
            .collect()
        },
        &level_tex,
    ));

    let animations: Vec<Animation> = vec![
        Animation {
            frames: vec![Rect {
                x: 24,
                y: 0,
                w: 20,
                h: 24,
            },
            Rect {
                x: 48,
                y: 0,
                w: 20,
                h: 24,
            },],
            times: vec![5, 5],
            looping: true,
        },
        Animation {
            frames: vec![Rect {
                x: 96,
                y: 0,
                w: 20,
                h: 24,
            },
            Rect {
                x: 120,
                y: 0,
                w: 20,
                h: 24,
            },
            Rect {
                x: 144,
                y: 0,
                w: 20,
                h: 24,
            },
            Rect {
                x: 168,
                y: 0,
                w: 20,
                h: 24,
            },
            Rect {
                x: 192,
                y: 0,
                w: 20,
                h: 24,
            },
            Rect {
                x: 216,
                y: 0,
                w: 20,
                h: 24,
            },],
            times: vec![3, 3, 3, 3, 3, 3],
            looping: true,
        },
    ];

    let levels: Vec<Level> = vec![
        (vec![Tilemap::new(
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
        ),
        Tilemap::new(
            Vec2i(0, 0),
            (10, 2),
            &tileset,
            vec![
                30, 30, 30, 30, 30, 5, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 13, 30, 30, 30, 30,
            ],
        )], vec![
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
            0,
            0,
            AnimationState::Nothing,
            Effect::Fight,
        )], 3),
        (vec![Tilemap::new(
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
        ),
        Tilemap::new(
            Vec2i(0, 0),
            (10, 2),
            &tileset,
            vec![
                30, 30, 30, 30, 30, 5, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 13, 30, 30, 30, 30,
            ],
        )], vec![
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
            0,
            0,
            AnimationState::Nothing,
            Effect::Fight
        )], 4),
        (vec![Tilemap::new(
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
        ),
        Tilemap::new(
            Vec2i(0, 0),
            (10, 2),
            &tileset,
            vec![
                30, 30, 30, 30, 30, 5, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 13, 30, 30, 30, 30,
            ],
        )], vec![
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
            0,
            0,
            AnimationState::Nothing,
            Effect::Fight,
        )], 5),
        (vec![Tilemap::new(
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
        )], vec![Sprite::new(
            &level_tex,
            Rect {
                x: 160,
                y: 64,
                w: 32,
                h: 32,
            },
            Vec2i(128, 128),
            true,
            0,
            0,
            AnimationState::Nothing,
            Effect::Win,
            ),
        ], 0),
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
            0,
            0,
            AnimationState::Standing_Right,
            Effect::Nothing
        ),
        health: HealthStatus{
            image: Rc::clone(&health_tex),
            lives: 5,
            frame: Rect {
                x:0,
                y:0,
                w:16,
                h:16
            },
            start: Vec2i(32, 56),
            spacing: 18
        },
        enemy_health: HealthStatus{
            image: Rc::clone(&health_tex),
            lives: levels[0].2,
            frame: Rect {
                x:0,
                y:0,
                w:16,
                h:16
            },
            start: Vec2i(240, 56),
            spacing: 18
        },
        player_choice: Attack::Nothing,
        enemy_choice: Attack::Nothing,
        choice_frame: 0,
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

            draw_game(&mut state, &mut screen, &levels, &animations, frame_count);

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

fn draw_game(state: &mut GameState, screen: &mut Screen, levels: &Vec<Level>, animations: &[Animation], frame: usize) {
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
            for m in levels[state.level].0.iter() {
                m.draw(screen);
            }
            if !state.passed {
                for (si, s) in levels[state.level].1.iter().enumerate() {
                    screen.draw_sprite(&s);
                }
            }

            // TODO: With reversed bitblt, reverse left facing animations
            state.player.frame = animations[state.player.animation].current_frame(state.player.animation_start, frame);
            screen.draw_sprite(&state.player);
        }
        GameMode::Fight => {
            let w = WIDTH as i32;
            let h = HEIGHT as i32;
            screen.draw_health(&state.health);
            screen.draw_health(&state.enemy_health);
            screen.bitblt(&state.player.image, state.player.frame, Vec2i(32, 32));
            screen.bitblt(&levels[state.level].1[0].image, levels[state.level].1[0].frame, Vec2i(264, 32));
            screen.rect(Rect { x: 32, y: 85, w: WIDTH as u16 /3, h: HEIGHT as u16 /6 }, Rgba(255, 255, 255, 255));
            screen.rect(Rect { x: 182, y: 85, w: WIDTH as u16 /3, h: HEIGHT as u16 /6 }, Rgba(255, 255, 255, 255));
            let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
            layout.reset(&LayoutSettings {
                x: 32 as f32,
                y: 85 as f32,
                max_height: Some((h/6) as f32),
                vertical_align: fontdue::layout::VerticalAlign::Middle,
                max_width: Some(((w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            match state.player_choice {
                Attack::Aggressive => { layout.append(&state.fonts.font_list, &TextStyle::new("aggressive", 20.0, 0)); }
                Attack::Defensive => { layout.append(&state.fonts.font_list, &TextStyle::new("defensive", 20.0, 0)); }
                Attack::Sneaky => { layout.append(&state.fonts.font_list, &TextStyle::new("sneaky", 20.0, 0)); }
                _ => {}
            }
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
            layout.reset(&LayoutSettings {
                x: 182 as f32,
                y: 85 as f32,
                max_height: Some((h/6) as f32),
                vertical_align: fontdue::layout::VerticalAlign::Middle,
                max_width: Some(((w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            match state.enemy_choice {
                Attack::Aggressive => { layout.append(&state.fonts.font_list, &TextStyle::new("aggressive", 20.0, 0)); }
                Attack::Defensive => { layout.append(&state.fonts.font_list, &TextStyle::new("defensive", 20.0, 0)); }
                Attack::Sneaky => { layout.append(&state.fonts.font_list, &TextStyle::new("sneaky", 20.0, 0)); }
                _ => {}
            }
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
        },
        GameMode::FightChoice => {
            let w = WIDTH as i32;
            let h = HEIGHT as i32;
            screen.draw_health(&state.health);
            screen.draw_health(&state.enemy_health);
            screen.bitblt(&state.player.image, state.player.frame, Vec2i(32, 32));
            screen.bitblt(&levels[state.level].1[0].image, levels[state.level].1[0].frame, Vec2i(264, 32));
            screen.rect(Rect { x: 32, y: 85, w: w as u16 /3, h: HEIGHT as u16 /6 }, Rgba(255, 255, 255, 255));
            screen.rect(Rect { x: 32, y: 135, w: w as u16 /3, h: HEIGHT as u16 /6 }, Rgba(255, 255, 255, 255));
            screen.rect(Rect { x: 32, y: 185, w: w as u16 /3, h: HEIGHT as u16 /6 }, Rgba(255, 255, 255, 255));
            screen.rect(Rect { x: 182, y: 85, w: w as u16 /3, h: HEIGHT as u16 /6 }, Rgba(0, 0, 0, 255));
            screen.rect(Rect { x: 182, y: 135, w: w as u16 /3, h: HEIGHT as u16 /6 }, Rgba(0, 0, 0, 255));
            screen.rect(Rect { x: 182, y: 185, w: w as u16 /3, h: HEIGHT as u16 /6 }, Rgba(0, 0, 0, 255));
            let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
            layout.reset(&LayoutSettings {
                x: 32 as f32,
                y: 85 as f32,
                max_height: Some((h/6) as f32),
                vertical_align: fontdue::layout::VerticalAlign::Middle,
                max_width: Some(((w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(&state.fonts.font_list, &TextStyle::new("[a]ggressive", 20.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
            layout.reset(&LayoutSettings {
                x: 32 as f32,
                y: 135 as f32,
                max_height: Some((h/6)  as f32),
                vertical_align: fontdue::layout::VerticalAlign::Middle,
                max_width: Some(((w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(&state.fonts.font_list, &TextStyle::new("[d]efensive", 20.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
            layout.reset(&LayoutSettings {
                x: 32 as f32,
                y: 185 as f32,
                max_height: Some((h/6)  as f32),
                vertical_align: fontdue::layout::VerticalAlign::Middle,
                max_width: Some(((w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(&state.fonts.font_list, &TextStyle::new("[s]neaky", 20.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
        }
        GameMode::GameOver => {
            state.window = Vec2i(0,0);

            let w = WIDTH as i32;
            let h = HEIGHT as i32;
            let menu_rect = Rect{x: w/6, y: h/8, w: (2*w as u16)/3, h: (h as u16)/2};

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
            layout.append(&state.fonts.font_list, &TextStyle::new("GAME\nOVER", 45.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (HEIGHT / 2) as f32,
                max_width: Some(((2*w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(&state.fonts.font_list, &TextStyle::new("Press ENTER to play again", 20.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
        },
        GameMode::Win => {
            state.window = Vec2i(0,0);
            let w = WIDTH as i32;
            let h = HEIGHT as i32;
            let menu_rect = Rect{x: w/6, y: h/8, w: (2*w as u16)/3, h: (h as u16)/2};

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
            layout.append(&state.fonts.font_list, &TextStyle::new("YOU\nWIN", 45.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (HEIGHT / 2) as f32,
                max_width: Some(((2*w)/3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(&state.fonts.font_list, &TextStyle::new("Press ENTER to play again", 20.0, 0));
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(250, 30, 10, 255),
            );
        }
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
                state.player.position.0 += 2;
                if state.player.animation_state != AnimationState::Walking_Right {
                    state.player.animation = 1;
                    state.player.animation_state = AnimationState::Walking_Right;
                    state.player.animation_start = frame;
                }
            } else if input.key_held(VirtualKeyCode::Left) {
                state.player.position.0 -= 2;
                if state.player.animation_state != AnimationState::Walking_Left {
                    state.player.animation = 1;
                    state.player.animation_state = AnimationState::Walking_Left;
                    state.player.animation_start = frame;
                }
            } else if input.key_held(VirtualKeyCode::Up) {
                state.player.position.1 -= 2;
                if state.player.animation_state != AnimationState::Walking_Left && state.player.animation_state != AnimationState::Walking_Right {
                    if state.player.animation_state == AnimationState::Standing_Right {
                        state.player.animation_state = AnimationState::Walking_Right;
                    }
                    if state.player.animation_state == AnimationState::Standing_Left {
                        state.player.animation_state = AnimationState::Walking_Left;
                    }
                    state.player.animation = 1;
                    state.player.animation_start = frame;
                }
            } else if input.key_held(VirtualKeyCode::Down) {
                state.player.position.1 += 2;
                if state.player.animation_state != AnimationState::Walking_Left && state.player.animation_state != AnimationState::Walking_Right {
                    if state.player.animation_state == AnimationState::Standing_Right {
                        state.player.animation_state = AnimationState::Walking_Right;
                    }
                    if state.player.animation_state == AnimationState::Standing_Left {
                        state.player.animation_state = AnimationState::Walking_Left;
                    }
                    state.player.animation = 1;
                    state.player.animation_start = frame;
                }
            } else {
                if state.player.animation_state != AnimationState::Standing_Left && state.player.animation_state != AnimationState::Standing_Right {
                    if state.player.animation_state == AnimationState::Walking_Right {
                        state.player.animation_state = AnimationState::Standing_Right;
                    }
                    if state.player.animation_state == AnimationState::Walking_Left {
                        state.player.animation_state = AnimationState::Standing_Left;
                    }
                    state.player.animation = 0;
                    state.player.animation_start = frame;
                }
            }

            // Detect collisions: See if the player is collided with an obstacle
            state.contacts.clear();
            let mut statics = &vec![];
            if !state.passed {
                statics = &levels[state.level].1;
            }
            gather_contacts(&levels[state.level].0[0], &state.player, statics, &mut state.contacts);

            match restitute(&levels[state.level].0[0], &mut state.player, statics, &mut state.contacts) {
                Effect::Fight => {
                    state.mode = GameMode::FightChoice;
                    
                },
                Effect::Win => { 
                    state.mode = GameMode::Win;
                },
                _ => {}
            }

            if state.player.position.0 < 192 && state.player.position.0 > 160 && state.player.position.1 < 8 {
                state.level += 1;
                state.player.position = Vec2i(128, 352);
                state.window = Vec2i(0, 128);
                state.passed = false;
            }

            if state.player.position.1 > (state.window.1 + HEIGHT as i32 - 32)  {
                state.window.1 += 2;
                if state.window.1 > HEIGHT as i32*2 {
                    state.window.1 = HEIGHT as i32*2;
                }
            }
            if state.player.position.1 < (state.window.1 + 32) {
                state.window.1 -= 2;
                if state.window.1 < 0 {
                    state.window.1 = 0;
                }
            }
        }
        GameMode::FightChoice => {
            let enemy_choice = get_enemy_decision(state.health.lives, state.enemy_health.lives, levels[state.level].2);
            let mut rng = rand::thread_rng();
            state.enemy_choice = enemy_choice;

            if input.key_held(VirtualKeyCode::Return) {
                state.mode = GameMode::Map;
                state.passed = true;
            }
            if input.key_released(VirtualKeyCode::A) {
                state.player_choice = Attack::Aggressive;
                match enemy_choice {
                    Attack::Aggressive => {},
                    Attack::Defensive => {
                        let decision = rng.gen_bool(0.5);
                        if decision && state.enemy_health.lives < levels[state.level].2 {
                            state.enemy_health.lives += 1;
                        }
                    },
                    Attack::Sneaky => {
                        if state.enemy_health.lives < 1 {
                            state.enemy_health.lives = 0;
                        } else {
                            state.enemy_health.lives -= 1;
                        }
                    }
                    _ => {}
                }
                state.mode = GameMode::Fight;
                state.choice_frame = frame;
            }
            if input.key_released(VirtualKeyCode::S) {
                state.player_choice = Attack::Sneaky;
                match enemy_choice {
                    Attack::Aggressive => {
                        if state.health.lives < 1 {
                            state.health.lives = 0;
                        } else {
                            state.health.lives -= 1;
                        }
                    },
                    Attack::Defensive => {
                        let decision = rng.gen_bool(0.25);
                        if decision {
                            if state.enemy_health.lives < 3 {
                                state.enemy_health.lives = 0;
                            } else {
                                state.enemy_health.lives -= 3;
                            }
                        }
                    },
                    Attack::Sneaky => {},
                    _ => {}
                }
                state.mode = GameMode::Fight;
                state.choice_frame = frame;
            }
            if input.key_released(VirtualKeyCode::D) {
                state.player_choice = Attack::Defensive;
                match enemy_choice {
                    Attack::Aggressive => {
                        let decision = rng.gen_bool(0.5);
                        if decision && state.health.lives < 5 {
                            state.health.lives += 1;
                        }
                    },
                    Attack::Defensive => {},
                    Attack::Sneaky => {
                        let decision = rng.gen_bool(0.25);
                        if decision {
                            if state.health.lives < 3 {
                                state.health.lives = 0;
                            } else {
                                state.health.lives -= 3;
                            }
                        }
                    },
                    _ => {}
                }
                state.mode = GameMode::Fight;
                state.choice_frame = frame;
            }
        }
        GameMode::Fight => {
            if frame - state.choice_frame > 120 {
                if state.enemy_health.lives == 0 {
                    state.enemy_health.lives = levels[state.level].2;
                    state.health.lives = 5;
                    state.mode = GameMode::Map;
                    state.passed = true;
                } else if state.health.lives == 0 {
                    state.mode = GameMode::GameOver;
                } else { 
                    state.mode = GameMode::FightChoice;
                }
            }
        }
        GameMode::GameOver => {
            if input.key_held(VirtualKeyCode::Return) {
                state.mode = GameMode::Map;
                reset_game(state, levels);
            }
        }
        GameMode::Win => {
            if input.key_held(VirtualKeyCode::Return) {
                state.mode = GameMode::Map;
                reset_game(state, levels);
            }
        }
    }
}

fn get_enemy_decision(player_health: usize, enemy_health: usize, enemy_cap: usize) -> Attack {
    let mut rng = rand::thread_rng();
    let decision = rng.gen_range(0, 100);
    if decision < 33 {
        Attack::Defensive
    } else if decision < 66 {
        Attack::Aggressive
    } else {
        Attack::Sneaky
    }
}

fn reset_game(state: &mut GameState, levels: &[Level]) {
    state.player.position = Vec2i(136, 224);
    state.health.lives = 5;
    state.enemy_health.lives = levels[0].2;
    state.player_choice = Attack::Nothing;
    state.enemy_choice = Attack::Nothing;
    state.choice_frame = 0;
    state.contacts.clear();
    state.window = Vec2i(0,0);
    state.level = 0;
    state.passed = false;
}

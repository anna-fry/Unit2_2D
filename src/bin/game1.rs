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
    animation::*, collision::*, health::*, screen::Screen, sprite::*, text::*, texture::Texture, tiles::*,
    types::*,
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
    obstacle_maps: Vec<Tilemap>,
    player_velocity: f32,
    scroll_speed: usize,
    scroll_timer: usize,
    map: Tilemap,
    health: HealthStatus,
    contacts: Vec<Contact>,
    immunities: Vec<isize>,
    fonts: Fonts,
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

    let font: &[u8] = &read(Path::new("content/monogram_font.ttf")).unwrap();
    let fonts = [Font::from_bytes(font, fontdue::FontSettings::default()).unwrap()];

    // TODO: Once we find the texture we want to use replace this path and delete the current placeholder file
    let tex = Rc::new(Texture::with_file(Path::new("content/penguin.png")));
    let tile_tex = Rc::new(Texture::with_file(Path::new("content/Background.png")));
    let health_tex = Rc::new(Texture::with_file(Path::new("content/Heart.png")));
    let obs_tex = Rc::new(Texture::with_file(Path::new("content/IceTileset.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile {
                solid: false,
                collide: Effect::Nothing,
            },
            Tile {
                solid: false,
                collide: Effect::Nothing,
            },
            Tile {
                solid: true,
                collide: Effect::Nothing,
            },
            Tile {
                solid: true,
                collide: Effect::Nothing,
            },
        ],
        &tile_tex,
    ));

    let obs_set = Rc::new(Tileset::new(
        //ice 1
        vec![
            //0:active ice
            Tile {
                solid: false,
                collide: Effect::Speedup(1),
            },
            //1:used ice
            Tile {
                solid: false,
                collide: Effect::Nothing,
            },
            //2: active rock
            Tile {
                solid: false,
                collide: Effect::Hurt(1),
            },
            //3: tree
            Tile {
                solid: false,
                collide: Effect::Hurt(1),
            },
            //4: ground
            Tile {
                solid: false,
                collide: Effect::Nothing,
            },
            //5: wall
            Tile {
                solid: true,
                collide: Effect::Nothing,
            },
            //6: nothing
            Tile {
                solid: false,
                collide: Effect::Nothing,
            },
        ],
        &obs_tex,
    ));
    let obstacle_map: Vec<Tilemap> = vec![
        Tilemap::new(Vec2i(TILE_SZ as i32, 0), (8, 5), &obs_set, vec![6; 40]),
        Tilemap::new(
            Vec2i(TILE_SZ as i32, 5 * TILE_SZ as i32),
            (8, 5),
            &obs_set,
            vec![6; 40],
        ),
        Tilemap::new(
            Vec2i(TILE_SZ as i32, 10 * TILE_SZ as i32),
            (8, 5),
            &obs_set,
            vec![6; 40],
        ),
        Tilemap::new(
            Vec2i(TILE_SZ as i32, 15 * TILE_SZ as i32),
            (8, 5),
            &obs_set,
            vec![6;40])];

    let animations: Vec<Animation> = vec![
        Animation {
            frames: vec![Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 16,
            },],
            times: vec![1],
            looping: true,
        },
        Animation {
            frames: vec![Rect {
                x: 16,
                y: 0,
                w: 16,
                h: 16,
            },],
            times: vec![1],
            looping: true,
        },
        Animation {
            frames: vec![Rect {
                x: 32,
                y: 0,
                w: 16,
                h: 16,
            },],
            times: vec![1],
            looping: true,
        },
        Animation {
            frames: vec![Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 16,
            },
            Rect {
                x: 48,
                y: 4,
                w: 16,
                h: 23,
            },],
            times: vec![1, 30],
            looping: false,
        }
    ];
    
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
            0,
            0,
            AnimationState::Facing_Forwad,
            Effect::Nothing,
        ),
        player_velocity: 0.0,
        scroll_speed: 2,
        scroll_timer: 180,
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
        obstacle_maps: obstacle_map,
        health: HealthStatus {
            image: Rc::clone(&health_tex),
            lives: 3,
            frame: Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 16,
            },
            start: Vec2i(300, 30),
            spacing: -18,
        },
        contacts: vec![],
        immunities: vec![0, 0],
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
            let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH, Vec2i(0, 0));
            screen.clear(Rgba(0, 0, 0, 0));

            draw_game(&mut state, &mut screen, frame_count, &animations);

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

fn draw_game(state: &mut GameState, screen: &mut Screen, frame: usize, animations: &[Animation]) {
    // Note: I had to make state mut to change the rasterized hashmap as needed
    // Call screen's drawing methods to render the game state
    screen.clear(Rgba(80, 80, 80, 255));

    match state.mode {
        GameMode::Title => {
            // draws menu screen
            state.map.draw(screen);

            let w = WIDTH as i32;
            let h = HEIGHT as i32;
            let menu_rect = Rect {
                x: w / 6,
                y: h / 8,
                w: (2 * w as u16) / 3,
                h: (h as u16) / 2,
            };

            screen.rect(menu_rect, Rgba(20, 0, 100, 255));
            screen.empty_rect(menu_rect, 4, Rgba(200, 220, 255, 255));

            let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (HEIGHT / 6) as f32,
                max_width: Some(((2 * w) / 3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(
                &state.fonts.font_list,
                &TextStyle::new("PENGUIN\nSLEDDING", 45.0, 0),
            );
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(255, 255, 255, 255),
            );
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (HEIGHT / 2) as f32,
                max_width: Some(((2 * w) / 3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(
                &state.fonts.font_list,
                &TextStyle::new("Press ENTER to start", 20.0, 0),
            );
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(255, 255, 255, 255),
            );
        }
        GameMode::Playing => {
            state.map.draw(screen);
            for map in state.obstacle_maps.iter() {
                map.draw(screen);
            }

            state.player.frame = animations[state.player.animation].current_frame(state.player.animation_start, frame);
            screen.draw_sprite(&state.player);
            screen.draw_health(&state.health);
        }
        GameMode::GameOver => {
            // draws game over screen
            state.map.draw(screen);

            let w = WIDTH as i32;
            let h = HEIGHT as i32;
            let menu_rect = Rect {
                x: w / 6,
                y: h / 8,
                w: (2 * w as u16) / 3,
                h: (h as u16) / 2,
            };

            screen.rect(menu_rect, Rgba(20, 0, 100, 255));
            screen.empty_rect(menu_rect, 4, Rgba(200, 220, 255, 255));

            let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (HEIGHT / 6) as f32,
                max_width: Some(((2 * w) / 3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(
                &state.fonts.font_list,
                &TextStyle::new("GAME\nOVER", 45.0, 0),
            );
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(255, 255, 255, 255),
            );
            layout.reset(&LayoutSettings {
                x: (WIDTH / 6) as f32,
                y: (HEIGHT / 2) as f32,
                max_width: Some(((2 * w) / 3) as f32),
                horizontal_align: fontdue::layout::HorizontalAlign::Center,
                ..LayoutSettings::default()
            });
            layout.append(
                &state.fonts.font_list,
                &TextStyle::new("Press ENTER to play again", 20.0, 0),
            );
            screen.draw_text(
                &mut state.fonts.rasterized,
                &state.fonts.font_list[0],
                &mut layout,
                Rgba(255, 255, 255, 255),
            );
        }
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
    let height: i32 = TILE_SZ as i32 * 5;
    for obs_map in state.obstacle_maps.iter_mut() {
        obs_map.position.1 -= state.scroll_speed as i32;
        //print!("\n pos: {}", obs_map.position.1);
        if obs_map.position.1 + height <= 0 {
            //offscreen, generate next segment
            obs_map.position.1 += height * 4;
            let mut map: Vec<usize> = vec![6; 40];
            for row in 0..4 {
                let mut num_obstacles = rng.gen_range(0, 4);
                let mut col = 0;
                while num_obstacles > 0 {
                    col = rng.gen_range(col, 8 - num_obstacles);
                    if rng.gen_bool(0.5) {
                        //stone
                        map[row * 8 + col] = 2;
                    } else {
                        //ice
                        map[row * 8 + col] = 0;
                    }
                    num_obstacles -= 1;
                }
            }
            obs_map.new_map(map);
        }
    }
}

fn update_tiles(state: &mut GameState) {
    state.map.position.1 -= state.scroll_speed as i32;
    if state.map.position.1.abs() >= TILE_SZ as i32 {
        state.map.position.1 = 0;
    }
}

fn update_game(state: &mut GameState, input: &WinitInputHelper, frame: usize) {
    match state.mode {
        GameMode::Title => {
            if input.key_held(VirtualKeyCode::Return) {
                state.mode = GameMode::Playing;
            }
        }
        GameMode::Playing => {
            // Player control goes here

            if input.key_held(VirtualKeyCode::Right) {
                state.player_velocity = (state.player_velocity + 0.75).min(3.0);
                if state.player.animation_state != AnimationState::Standing_Right && state.player.animation_state != AnimationState::Fallen {
                    state.player.animation = 2;
                    state.player.animation_state = AnimationState::Standing_Right;
                    state.player.animation_start = frame;
                }
            } else if input.key_held(VirtualKeyCode::Left) {
                state.player_velocity = (state.player_velocity - 0.75).max(-3.0);
                if state.player.animation_state != AnimationState::Standing_Left && state.player.animation_state != AnimationState::Fallen{
                    state.player.animation = 1;
                    state.player.animation_state = AnimationState::Standing_Left;
                    state.player.animation_start = frame;
                }
            } else {
                if state.player_velocity> 0.0{
                    state.player_velocity -= 0.1;
                    state.player_velocity = state.player_velocity.max(0.0);
                }
                else if state.player_velocity< 0.0{
                    state.player_velocity += 0.1;
                    state.player_velocity = state.player_velocity.min(0.0);
                }
                if state.player.animation_state != AnimationState::Facing_Forwad && state.player.animation_state != AnimationState::Fallen {
                    state.player.animation = 0;
                    state.player.animation_state = AnimationState::Facing_Forwad;
                    state.player.animation_start = frame;
                }
            }
            if state.scroll_timer == 0{
                //bootleg animation forcing: fallen state should only exist 
                //until the scroll timer changes the speed to 1
                if state.player.animation_state == AnimationState::Fallen{
                    state.player.animation = 0;
                    state.player.animation_state = AnimationState::Facing_Forwad;
                    state.player.animation_start = frame;
                }
                state.scroll_timer = 180;
                state.scroll_speed +=1;
            }
            else{
                state.scroll_timer -=1;
            }
            state.player.position.1 = 30;
            state.player.position.0 += state.player_velocity as i32;

            // Scroll the scene
            update_obstacles(state);
            update_tiles(state);

            // Detect collisions: See if the player is collided with an obstacle
            state.contacts.clear();
            gather_contacts(&state.map, &state.player, &[], &mut state.contacts);

            // Detect collisions: See if the player is collided with an obstacle
            state.contacts.clear();
            gather_contacts(&state.map, &state.player, &[], &mut state.contacts);
            for vec in state.obstacle_maps.iter() {
                gather_contacts(&vec, &state.player, &[], &mut state.contacts);
            }
            // Handle collisions: Take damage, speed up, or slow down
            state.immunities[0] -= 1;
            state.immunities[1] -= 1;
            match restitute(&state.map, &mut state.player, &[], &mut state.contacts) {
                //match collision_effect(&state.player, &mut state.obstacles){
                Effect::Hurt(n) => {
                    if state.immunities[0] <= 0 {
                        state.player.animation = 3;
                        state.player.animation_state = AnimationState::Fallen;
                        state.player.animation_start = frame;
                        update_obstacles(state);
                        update_tiles(state);                
                        if state.health.lives > n {
                            state.immunities[0] = 100;
                            state.scroll_timer = 15;
                            state.health.lives -= n;
                            state.scroll_speed = 0;
                        } else {
                            state.mode = GameMode::GameOver;
                        }
                    }
                }
                Effect::Speedup(n) => {
                    if state.immunities[1] <= 0 {
                        state.scroll_speed += n;
                        state.immunities[1] = 60;
                    }
                }
                _ => {}
            }
        }
        GameMode::GameOver => {
            if input.key_held(VirtualKeyCode::Return) {
                state.mode = GameMode::Playing;
                reset_game(state);
            }
        }
    }
}

/**
 *  Resets game to a beginning state
**/
fn reset_game(state: &mut GameState) {
    state.player.position = Vec2i(160, 20);
    state.health.lives = 3;
    state.contacts.clear();
    state.scroll_speed = 2;
    state.scroll_timer = 180;
    state.player_velocity = 0.0;
    for map in state.obstacle_maps.iter_mut() {
        map.new_map(vec![6; 40]);
    }
    // for ob in state.obstacles.iter_mut() {
    //     ob.drawable = false;

    // }
}

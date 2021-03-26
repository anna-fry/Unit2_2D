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

//TODO: Fill out state
// The State needs to keep track of the player...
// Add texture when we decide on the texture we want
struct GameState {
    player: Sprite,
    obstacles: Vec<Sprite>,
    obstacle_maps: Vec<Tilemap>,
    spawn_timer: isize,
    scroll_speed: usize,
    map: Tilemap,
    health: HealthStatus,
    contacts: Vec<Contact>,
    immunities: Vec<isize>
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
    let tex = Rc::new(Texture::with_file(Path::new("content/penguin.png")));
    let tile_tex = Rc::new(Texture::with_file(Path::new("content/Background.png")));
    let health_tex = Rc::new(Texture::with_file(Path::new("content/Heart.png")));
    let obs_tex = Rc::new(Texture::with_file(Path::new("content/IceTileset.png")));
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false, collide:Effect::Nothing },
            Tile { solid: false, collide:Effect::Nothing },
            Tile { solid: true, collide:Effect::Nothing },
            Tile { solid: true, collide:Effect::Nothing },
        ],
        &tile_tex,
    ));
    /*
    let obs_set =Rc::new(Tileset::new(
        vec![
            //0: tree top
            Tile { solid: false, collide:Effect::Hurt(2) },
            //1: tree side?
            Tile { solid: false, collide:Effect::Hurt(2)},
            //2: sign
            Tile { solid: false, collide:Effect::Nothing},
            //3: log
            Tile { solid: false, collide:Effect::Hurt(1)},
            //4: tree bot
            Tile { solid: false, collide:Effect::Hurt(2) },
            //5: small bush
            Tile { solid: false, collide:Effect::Hurt(1)},
            //6: rock
            Tile { solid: false, collide:Effect::Hurt(1) },
            //7: nothing
            Tile { solid: false, collide:Effect::Nothing },
            //8: snow          
            Tile { solid: false, collide:Effect::Nothing },
            //9 ice
            Tile { solid: false, collide:Effect::Speedup(1)},
            //10 stone
            Tile { solid: true, collide:Effect::Hurt(1) },
            //11 brick
            Tile { solid: true, collide:Effect::Hurt(1) },


        ]
        ,&obs_tex));
    */
    let obs_set = Rc::new(Tileset::new(
        //ice 1
        vec![
            //0:active ice
            Tile { solid: false, collide:Effect::Speedup(1) },
            //1:used ice
            Tile { solid: false, collide:Effect::Nothing },
            //2: active rock
            Tile { solid: false, collide:Effect::Hurt(1) },
            //3: tree
            Tile { solid: false, collide:Effect::Hurt(1) },
            //4: ground
            Tile { solid: false, collide:Effect::Nothing },
            //5: wall
            Tile { solid: true, collide:Effect::Nothing },
            //6: nothing
            Tile { solid: false, collide:Effect::Nothing },


        ],
        &obs_tex));
    let obstacle_map:Vec<Tilemap> = vec![
        Tilemap::new(
        Vec2i(TILE_SZ as i32, 0),
        (8, 5),
        &obs_set,
        vec![6;40]),
        Tilemap::new(
            Vec2i(TILE_SZ as i32, 5*TILE_SZ as i32),
            (8, 5),
            &obs_set,
            vec![6;40]),
        Tilemap::new(
            Vec2i(TILE_SZ as i32, 10*TILE_SZ as i32),
            (8, 5),
            &obs_set,
            vec![6;40]),
        Tilemap::new(
            Vec2i(TILE_SZ as i32, 15*TILE_SZ as i32),
            (8, 5),
            &obs_set,
            vec![6;40])];


    
    let mut state = GameState {
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
        obstacles: vec![Sprite::new(&obs_tex, Rect{x:0, y:0, w:32, h:32}, Vec2i(100, 0), false); 10],
        spawn_timer: 0,
        scroll_speed: 3,
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
        ),
        obstacle_maps: obstacle_map,
        health: HealthStatus{
            image: Rc::clone(&health_tex),
            lives: 3,
            frame: Rect {
                x:0,
                y:0,
                w:16,
                h:16
            },
            start: Vec2i(300, 15),
            spacing: -18
        },
        contacts: vec![],
        immunities:vec![0,0]
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
    for map in state.obstacle_maps.iter(){
        map.draw(screen);
    }
    // TODO: Draw Sprites
    screen.draw_sprite(&state.player);
    // for sprite in state.obstacles.iter() {
    //     screen.draw_sprite(sprite);
    // }
    screen.draw_health(&state.health);
}
/**
 * updates all obstacles on screen:
 *  scrolls up
 *  removes obstacles over top of screen
 *  if new obstacles are needed, adds them
 */
fn update_obstacles(state: &mut GameState){
    let mut rng = rand::thread_rng();
    let height:i32 = TILE_SZ as i32 * 5 ;
    for obs_map in state.obstacle_maps.iter_mut(){
        obs_map.position.1 -= state.scroll_speed as i32;
        //print!("\n pos: {}", obs_map.position.1);
        if obs_map.position.1 + height <=0{
            //offscreen, generate next segment
            obs_map.position.1 += height * 4;
            let mut map:Vec<usize> = vec![6;40];
            for row in 0..4{
                let mut num_obstacles = rng.gen_range(0,4);
                let mut col = 0;
                while num_obstacles >0{
                    col = rng.gen_range(col, 8-num_obstacles);
                    if rng.gen_bool(0.5){
                        //stone
                        map[row*8 + col] = 2;
                    }
                    else{
                        //ice
                        map[row*8+col] = 0;
                    }
                    num_obstacles -=1;
                }
            }
            print!("\n old:{:?}", obs_map.map);
            obs_map.new_map(map);
            print!("\n new:{:?}", obs_map.map);

        
        }
    }

    // for sprite in state.obstacles.iter_mut(){
    //     if sprite.drawable{
    //         sprite.position.1 -= state.scroll_speed as i32;

    //         if sprite.position.1<=0{
    //             sprite.position.0 = rng.gen_range(1, WIDTH as i32/32) *31;
    //             sprite.position.1 = HEIGHT as i32 - 16;
    //             sprite.drawable = false;
    //         }
    //     }
    // }
    
    // if state.spawn_timer <=0{
    //     let mut flipped =false;
    //     for sprite in state.obstacles.iter_mut(){
    //         if !sprite.drawable && !flipped{
    //             sprite.drawable = true;
    //             flipped = rng.gen_range(0,5)<3;
    //             if rng.gen_bool(0.2){
    //                 sprite.frame.x = 32;
    //                 sprite.frame.y = 64;
    //                 sprite.collision = Effect::Speedup(1);
    //             }
    //             else{
    //                 sprite.frame.x = 64;
    //                 sprite.frame.y = 32;
    //                 sprite.collision = Effect::Hurt(1);
    //             }
    //             //TODO: make obstacles not spawn Together
    //             //TODO: make diff types of obstacles spawn
    //         }
    //     }
    //     state.spawn_timer =rng.gen_range(16, 50);
    // }
    state.spawn_timer -= state.scroll_speed as isize;
}

fn update_tiles(state: &mut GameState){
    state.map.position.1 -= state.scroll_speed as i32;
    if state.map.position.1.abs() >= TILE_SZ  as i32 {
        state.map.position.1 = 0;
    }

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

    // Make sure player stays at same height
    state.player.position.1 = 30;

    // Scroll the scene
    update_obstacles(state);
    update_tiles(state);


    // Detect collisions: See if the player is collided with an obstacle
    state.contacts.clear();
    gather_contacts(&state.map, &state.player, &mut state.contacts);
    for vec in state.obstacle_maps.iter(){
        gather_contacts(&vec, &state.player, &mut state.contacts);
    }
    // TODO: Handle collisions: Take damage, speed up, or slow down
    state.immunities[0] -= 1;
    state.immunities[1] -=1;
    match restitute(&state.map, &mut state.player, &mut state.contacts){
    //match collision_effect(&state.player, &mut state.obstacles){
        Effect::Hurt(n) => 
        {   if state.immunities[0]<=0{
                if state.health.lives >n{
                    state.immunities[0] =48;
                    state.health.lives -=n;
                    state.scroll_speed =1;}
                else{
                    //TODO: make this an end of game
                    state.health.lives =0;
                    state.scroll_speed =0;
            }
        }
    },
        Effect::Speedup(n) => {
            if state.immunities[1] <=0{
                state.scroll_speed +=n;
                state.immunities[1] = 48;
            }
            },
        _ => {}
    }
}

use std::any::Any;
use std::rc::Rc;

// // TODO: Compare pos of obstacles and the player from the state and change velocity/health based on collision
use crate::tiles::*;
use crate::types::*;
use crate::{sprite::{Sprite,Effect}, tiles::Tilemap};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ColliderID {
    Static((usize, Vec2i)),
    Dynamic(usize),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Contact {
    a: ColliderID,
    b: ColliderID,
    mtv: (i32, i32),
}

// Only looks for collision btw a single sprite and a single tilemap rn
pub fn gather_contacts(
    tilemap: &Tilemap,
    sprite: &Sprite,
    into: &mut Vec<Contact>,
) {
    // collide mobiles against walls
    // Checks tiles at the corners
    let corners = vec![
        (sprite.position.0, sprite.position.1),
        (sprite.position.0 + sprite.get_dimensions().0, sprite.position.1),
        (
            sprite.position.0 + sprite.get_dimensions().0,
            sprite.position.1 + sprite.get_dimensions().1,
        ),
        (sprite.position.0, sprite.position.1 + sprite.get_dimensions().1),
    ];
    for (x, y) in corners {
        if x >= tilemap.position.0
            && x < tilemap.position.0 + (TILE_SZ * tilemap.size().0) as i32
            && y >= tilemap.position.1
            && y < tilemap.position.1 + (TILE_SZ * tilemap.size().1) as i32
        {
            let (tile, pos) = tilemap.tile_at(Vec2i(x, y));
            if tile.solid {
                // Treat the tile and sprite as rectangles
                let a_rect = Rect {
                    x: sprite.position.0,
                    y: sprite.position.1,
                    w: sprite.get_dimensions().0 as u16,
                    h: sprite.get_dimensions().1 as u16,
                };
                let rect = Rect {
                    // position on the whole map
                    x: pos.0 * TILE_SZ as i32 + tilemap.position.0,
                    y: pos.1 * TILE_SZ as i32 + tilemap.position.1,
                    w: TILE_SZ as u16,
                    h: TILE_SZ as u16,
                };
                let mtv = Rect::rect_displacement(a_rect, rect);
                if let Some(m) = mtv {
                    into.push(Contact {
                        a: ColliderID::Dynamic(0),
                        b: ColliderID::Static((0, Vec2i(x, y))),
                        mtv: m,
                    });
                }
            }
        }
    }
}

pub fn collision_effect(sprite: &Sprite, obstacles: &mut Vec<Sprite>) -> Effect{
    let mut effect:Effect = Effect::Nothing;
    let sprite_rect = Rect {
        x: sprite.position.0,
        y: sprite.position.1,
        w: sprite.get_dimensions().0 as u16,
        h: sprite.get_dimensions().1 as u16,
    };
    for obstacle in obstacles.iter_mut(){
        let obs_rect = Rect{
            x: obstacle.position.0,
            y: obstacle.position.1,
            w: obstacle.get_dimensions().0 as u16,
            h: obstacle.get_dimensions().1 as u16,
        };
        if Rect::rect_touching(sprite_rect, obs_rect){
            match obstacle.collision{
            Effect::Hurt(n) => {
                obstacle.collision = Effect::Hurt((n.max(1)-1));
                return Effect::Hurt(n);
            },
            Effect::Speedup(n) => {
                obstacle.collision = Effect::Hurt((n.max(1)-1));
                effect = Effect::Speedup(n)},
            _ => {}
            }
        }   
}
return effect;
}

pub fn restitute(
    tilemap: &Tilemap,
    sprite: &mut Sprite,
    contacts: &mut [Contact],
) {
    // handle restitution of dynamics against statics wrt contacts.
    // Assuming everything is rectangles
    for contact in contacts {
        // assume dynamic/static collision
        if let ColliderID::Dynamic(i) = contact.a {
            if let ColliderID::Static(si) = contact.b {
                let tile = tilemap.tile_at(si.1);
                let a_rect = Rect {
                    x: sprite.position.0,
                    y: sprite.position.1,
                    w: sprite.get_dimensions().0 as u16,
                    h: sprite.get_dimensions().1 as u16,
                };
                let rect = Rect {
                    x: tile.1 .0 * TILE_SZ as i32 + tilemap.position.0,
                    y: tile.1 .1 * TILE_SZ as i32 + tilemap.position.1,
                    w: TILE_SZ as u16,
                    h: TILE_SZ as u16,
                };

                if Rect::rect_touching(a_rect, rect) {
                    if contact.mtv.0 > contact.mtv.1 && contact.mtv.1 != 0 {
                        // move in y direction
                        match a_rect.y.cmp(&rect.y) {
                            std::cmp::Ordering::Greater => {
                                sprite.position.1 += contact.mtv.1;
                            }
                            std::cmp::Ordering::Less => {
                                sprite.position.1 -= contact.mtv.1;
                            }
                            std::cmp::Ordering::Equal => (),
                        }
                    //sprite.vy = 0.0;
                    } else if contact.mtv.0 <= contact.mtv.1 && contact.mtv.0 != 0 {
                        // move in x direction
                        match a_rect.x.cmp(&rect.x) {
                            std::cmp::Ordering::Greater => {
                                sprite.position.0 += contact.mtv.0;
                            }
                            std::cmp::Ordering::Less => {
                                sprite.position.0 -= contact.mtv.0;
                            }
                            std::cmp::Ordering::Equal => (),
                        }
                        //dynamics[i].vx = 0.0;
                    }
                }
            }
        }
    }
}

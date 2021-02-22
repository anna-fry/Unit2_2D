// // TODO: Compare pos of obstacles and the player from the state and change velocity/health based on collision
use crate::sprite::Sprite;
use crate::types::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ColliderID {
    Static(usize),
    Dynamic(usize),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Contact {
    a: ColliderID,
    b: ColliderID,
    mtv: (i32, i32),
}

// TODO: Adjust for how we're going to represent bounding boxes
fn gather_contacts(statics: &Vec<Sprite>, dynamics: &Vec<Sprite>, into: &mut Vec<Contact>) {
    // collide mobiles against mobiles
    for (ai, a) in dynamics.iter().enumerate() {
        for (bi, b) in dynamics.iter().enumerate() {
            let a_rect = Rect {
                x: a.position.0,
                y: a.position.1,
                w: a.frame.w,
                h: a.frame.h,
            };
            let b_rect = Rect {
                x: b.position.0,
                y: b.position.1,
                w: b.frame.w,
                h: b.frame.h,
            };
            if Rect::rect_touching(a.frame, b.frame) {
                let mtv = Rect::rect_displacement(a_rect, b_rect);
                if let Some(x) = mtv {
                    into.push(Contact {
                        a: ColliderID::Dynamic(ai),
                        b: ColliderID::Dynamic(bi),
                        mtv: x,
                    });
                }
            }
        }
    }
    // collide mobiles against walls
    for (ai, a) in dynamics.iter().enumerate() {
        for (bi, b) in statics.iter().enumerate() {
            let a_rect = Rect {
                x: a.position.0,
                y: a.position.1,
                w: a.frame.w,
                h: a.frame.h,
            };
            let b_rect = Rect {
                x: b.position.0,
                y: b.position.1,
                w: b.frame.w,
                h: b.frame.h,
            };
            if Rect::rect_touching(a_rect, b_rect) {
                let mtv = Rect::rect_displacement(a_rect, b_rect);
                if let Some(x) = mtv {
                    into.push(Contact {
                        a: ColliderID::Dynamic(ai),
                        b: ColliderID::Static(bi),
                        mtv: x,
                    });
                }
            }
        }
    }
}

// fn restitute(statics: &[Wall], dynamics: &mut [Mobile], contacts: &mut [Contact]) {
//     // handle restitution of dynamics against statics wrt contacts.
//     // Assuming everything is rectangles
//     for contact in contacts {
//         // assume dynamic/static collision
//         if let ColliderID::Dynamic(i) = contact.a {
//             if let ColliderID::Static(si) = contact.b {
//                 if Rect::rect_touching(dynamics[i].rect, statics[si].rect) {
//                     if contact.mtv.0 > contact.mtv.1 && contact.mtv.1 != 0 {
//                         // move in y direction
//                         match dynamics[i].rect.y.cmp(&statics[si].rect.y) {
//                             std::cmp::Ordering::Greater => {
//                                 dynamics[i].rect.y += contact.mtv.1;
//                             }
//                             std::cmp::Ordering::Less => {
//                                 dynamics[i].rect.y -= contact.mtv.1;
//                             }
//                             std::cmp::Ordering::Equal => (),
//                         }
//                         dynamics[i].vy = 0.0;
//                     } else if contact.mtv.0 <= contact.mtv.1 && contact.mtv.0 != 0 {
//                         // move in x direction
//                         match dynamics[i].rect.x.cmp(&statics[si].rect.x) {
//                             std::cmp::Ordering::Greater => {
//                                 dynamics[i].rect.x += contact.mtv.0;
//                             }
//                             std::cmp::Ordering::Less => {
//                                 dynamics[i].rect.x -= contact.mtv.0;
//                             }
//                             std::cmp::Ordering::Equal => (),
//                         }
//                         dynamics[i].vx = 0.0;
//                     }
//                 }
//             }
//         }
//     }
// }

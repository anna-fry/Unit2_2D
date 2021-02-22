// // TODO: Compare pos of obstacles and the player from the state and change velocity/health based on collision

// fn gather_contacts(statics: &[Wall], dynamics: &[Mobile], into: &mut Vec<Contact>) {
//     // collide mobiles against mobiles
//     for (ai, a) in dynamics.iter().enumerate() {
//         for (bi, b) in dynamics.iter().enumerate().skip(ai + 1) {
//             if Rect::rect_touching(a.rect, b.rect) {
//                 let mtv = Rect::rect_displacement(a.rect, b.rect);
//                 if let Some(x) = mtv {
//                     into.push(Contact {
//                         a: ColliderID::Dynamic(ai),
//                         b: ColliderID::Dynamic(bi),
//                         mtv: x,
//                     });
//                 }
//             }
//         }
//     }
//     // collide mobiles against walls
//     for (ai, a) in dynamics.iter().enumerate() {
//         for (bi, b) in statics.iter().enumerate() {
//             if Rect::rect_touching(a.rect, b.rect) {
//                 let mtv = Rect::rect_displacement(a.rect, b.rect);
//                 if let Some(x) = mtv {
//                     into.push(Contact {
//                         a: ColliderID::Dynamic(ai),
//                         b: ColliderID::Static(bi),
//                         mtv: x,
//                     });
//                 }
//             }
//         }
//     }
// }

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
use crate::types::Rect;
use std::rc::Rc;
use std::time::{Duration, Instant};


pub struct Animation {
    pub frames: Vec<Rect>,
    pub times: Vec<usize>,
    pub looping: bool,
}

#[derive(PartialEq, Clone)]
pub enum AnimationState {
    Standing_Left,
    Standing_Right,
    Walking_Left,
    Walking_Right,
    Facing_Forwad,
    Fallen,
    Nothing,
}

impl Animation {
    pub fn current_frame(&self, start_time: usize, now: usize) -> Rect {
        if self.looping {
            let net_duration: usize = self.times.iter().sum();
            let t = (now - start_time);
            let frame_disp: usize = t%net_duration;
            let mut frames_past = 0;
            let mut frame_index = 0;
            for time in self.times.iter() {
                frames_past += time;
                if frames_past > frame_disp {
                    break;
                }
                frame_index += 1;
            }
            self.frames[frame_index]
        } else if (now-start_time) >= self.times.iter().sum() {
            self.frames[0]
        } else {
            let frame_disp = (now - start_time);
            let mut frames_past = 0;
            let mut frame_index = 0;
            for time in self.times.iter() {
                frames_past += time;
                if frames_past > frame_disp {
                    break;
                }
                frame_index += 1;
            }
            self.frames[frame_index]
        }
    }
}

// Basic scripted movements to test coordinate translation
use std::sync::mpsc::Sender;
use crate::point::{RawPoint, CartesianPoint, OUTER_ARM_LENGTH, INNER_ARM_LENGTH};

pub fn scripted_mover(pos_sender: Sender<RawPoint>) {
    for _ in 0..100 {
        for i in 0..(INNER_ARM_LENGTH as u32) {
            let cpoint = CartesianPoint{ x: OUTER_ARM_LENGTH, y: (i as f32), z: 0.0};
            let rpoint = RawPoint::from(cpoint);
            pos_sender.send(rpoint).expect("Error sending point");
        }
        for i in (0..(INNER_ARM_LENGTH as u32)).rev() {
            let cpoint = CartesianPoint{ x: OUTER_ARM_LENGTH, y: (i as f32), z: 0.0};
            let rpoint = RawPoint::from(cpoint);
            pos_sender.send(rpoint).expect("Error sending point");
        }
        for i in 0..(OUTER_ARM_LENGTH as u32) {
            let cpoint = CartesianPoint{ x: (i as f32), y: INNER_ARM_LENGTH, z: 0.0};
            let rpoint = RawPoint::from(cpoint);
            pos_sender.send(rpoint).expect("Error sending point");
        }
        for i in (0..(OUTER_ARM_LENGTH as u32)).rev() {
            let cpoint = CartesianPoint{ x: (i as f32), y: INNER_ARM_LENGTH, z: 0.0};
            let rpoint = RawPoint::from(cpoint);
            pos_sender.send(rpoint).expect("Error sending point");
        }
    }
}

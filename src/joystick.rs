extern crate sdl2;

use std::sync::mpsc::Sender;
use crate::point::{RawPoint, LIMITS};

const ANALOG_DEADSPOT: i16 = 1000;

pub fn joystick_controller(pos_channel: Sender<RawPoint>) {
    let sdl_context = sdl2::init().expect("Error initializing SDL");

    let _controller = get_game_controller(&sdl_context).expect("Unable to connect to game controller");


    let mut speed = RawPoint { base: 0.0, lower: 0.0, upper: 0.0, claw: 0.0 };
    let mut pos = RawPoint {base: 90.0, lower: 90.0, upper: 90.0, claw: 90.0 };
    let mut last_pos = pos;


    let update_millis = 25;
    let max_speed: f32 = 1.0 / (update_millis as f32 / 1000.0);
    loop {
        for event in sdl_context.event_pump().expect("unable to get sdl event pump").poll_iter() {
            use sdl2::event::Event;
            use sdl2::controller::Button;
            use sdl2::controller::Axis;

            match event {
                Event::ControllerAxisMotion {
                    axis, value: val, ..
                } => {
                    // Axis motion is an absolute value in the range
                    // [-32768, 32767]. Let's simulate a very rough dead
                    // zone to ignore spurious events.
                    //

                    let stick_speed = {
                        if val > ANALOG_DEADSPOT || val < -ANALOG_DEADSPOT {
                            (val as f32 / 32767.0) * max_speed
                        } else { 0.0 }
                    };
                    println!("Axis {:?} moved to {}", axis, val);

                    match axis {
                        Axis::LeftX => speed.base = stick_speed,
                        Axis::LeftY => speed.lower = stick_speed,
                        Axis::RightY => speed.upper = stick_speed,
                        Axis::RightX => speed.claw = stick_speed,
                        _ => (),
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    match button {
                        Button::A => {
                            println!("It's the A button!");
                            if pos.claw != LIMITS[3].1 {
                                pos.claw = LIMITS[3].1;
                            } else {
                                pos.claw = LIMITS[3].0;
                            }
                        }
                        Button::Guide => {
                            println!("Setting position to home");
                            pos = RawPoint{ base: 90.0, upper: 90.0, lower: 90.0, claw: 90.0 };
                            speed = RawPoint{ base: 0.0, upper: 0.0, lower: 0.0, claw: 0.0 };
                        }
                        _ => (),
                    }

                    println!("Button {:?} down", button)
                }
                Event::ControllerButtonUp { button, .. } => println!("Button {:?} up", button),
                Event::Quit { .. } => break,
                _ => (),
            }

            pos.base = (pos.base + speed.base).max(LIMITS[0].0).min(LIMITS[0].1);
            pos.lower = (pos.lower + speed.lower).max(LIMITS[1].0).min(LIMITS[1].1);
            pos.upper = (pos.upper + speed.upper).max(LIMITS[2].0).min(LIMITS[2].1);
            pos.claw = (pos.claw + speed.claw).max(LIMITS[3].0).min(LIMITS[3].1);


            if pos != last_pos {
                pos_channel.send(pos).expect("Error sending point");
                last_pos = pos;
            }
        }
    }

}

fn get_game_controller(sdl_context: &sdl2::Sdl) -> Result<sdl2::controller::GameController, String> {
    let game_controller_subsystem = sdl_context.game_controller()?;
    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let controller = (0..available)
        .find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    Some(c)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        })
        .expect("Couldn't open any controller");

    return Ok(controller)
}

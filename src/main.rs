extern crate sdl2;
use std::time::{Duration, Instant};
use std::str;

const LIMITS: [(f32, f32); 3] = [
    (0.0, 180.0),  // Base rotation
    (60.0,150.0),  // Lower joint
    (40.0, 120.0), // Upper joint
];

fn main() -> Result<(), String> {
    ctrlc::set_handler( move || {std::process::exit(0)} ).expect("Error setting handler");
    let sdl_context = sdl2::init()?;
    let game_controller_subsystem = sdl_context.game_controller()?;

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);


    let mut port = serialport::new("/dev/ttyUSB1", 57600)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let update_millis = 50;
    let update_time = Duration::from_millis(update_millis);
    let max_speed: f32 = 2.0 / (update_millis as f32 / 1000.0);
    println!("{}", max_speed);
    let mut last_write_time = Instant::now();

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

    let mut pos: [f32; 3] = [90.0, 90.0, 90.0];
    let mut speed: [f32; 3] = [0.0, 0.0, 0.0];

    println!("Controller mapping: {}", controller.mapping());

    loop {
        for event in sdl_context.event_pump()?.poll_iter() {
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
                        if val > 100 || val < 100 {
                            (val as f32 / 32767.0) * max_speed
                        } else { 0.0 }
                    };
                    println!("Axis {:?} moved to {}", axis, val);

                    match axis {
                        Axis::LeftX => speed[0] = stick_speed,
                        Axis::LeftY => speed[2] = stick_speed,
                        Axis::RightY => speed[1] = stick_speed,
                        _ => (),
                    }
                }
                Event::ControllerButtonDown { button, .. } => {
                    match button {
                        Button::A => {
                            println!("It's the A button!");
                            pos = [130.0, 130.0, 130.0]

                        }
                        Button::B => {
                            pos = [90.0, 90.0, 90.0];
                        }
                        _ => (),
                    }

                    println!("Button {:?} down", button)
                }
                Event::ControllerButtonUp { button, .. } => println!("Button {:?} up", button),
                Event::Quit { .. } => break,
                _ => (),
            }
        }
        //println!("End position: {:?}", pos);
        // println!("Speed: {:?}", speed);
        if last_write_time.elapsed() > update_time {
            for i in 0..3 {
                // Set position to position plus speed OR the axis limit
                let mut new_pos = pos[i] + speed[i];
                new_pos = new_pos.max(LIMITS[i].0);
                new_pos = new_pos.min(LIMITS[i].1);

                pos[i] = new_pos;
            }

            println!("New position is {} {} {}", pos[0], pos[1], pos[2]);
            let output_bytes = [pos[0] as u8, pos[1] as u8, pos[2] as u8];
            port.write(&output_bytes).expect("write failed");
            last_write_time = Instant::now();
        }

        let mut serial_buf: Vec<u8> = vec![0; 32];
        match port.read(serial_buf.as_mut_slice()) {
            Ok (_) => print!("{}", str::from_utf8(&serial_buf).expect("string conversion failed")),
            Err (_) => (),
        };


    }
}

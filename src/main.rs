mod joystick;
mod point;
mod serial;
mod scripted;

extern crate sdl2;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() -> Result<(), String> {
    ctrlc::set_handler( move || {std::process::exit(0)} ).expect("Error setting handler");

    let (pos_sender, pos_receiver): (Sender<point::RawPoint>, Receiver<point::RawPoint>) = mpsc::channel();

    let port_name = serial::find_port().expect("No port found");
    let port_name2 = port_name.clone();

    let reader_thread = thread::spawn(move || serial::serial_reader(port_name));
    let updater_thread = thread::spawn(move || serial::position_updater(port_name2, pos_receiver));


    let joystick_enabled = false;
    let mover_thread: thread::JoinHandle<()>;
    if joystick_enabled {
        mover_thread = thread::spawn(move || joystick::joystick_controller(pos_sender));
    } else {
        mover_thread = thread::spawn(move || scripted::scripted_mover(pos_sender));
    }

    reader_thread.join().expect("oh no! error in the reader thread!");
    updater_thread.join().expect("oh no! error in the updater thread!");
    mover_thread.join().expect("oh no! error in the mover thread!");

    Ok(())
}

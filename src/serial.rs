use std::thread;
use std::time::Duration;
use std::str;
use std::sync::mpsc::Receiver;

use crate::point::RawPoint;

const SERIAL_BAUDRATE: u32 = 57600;

pub fn find_port() -> Result<String, String> {
    let available_ports = serialport::available_ports().expect("No ports found!");
    match available_ports.iter().find(|x| x.port_name.contains("ttyUSB")) {
        Some(port_info) => Ok(port_info.port_name.clone()),
        None => Err("No serial port found".to_string()),
    }
}

pub fn serial_reader(port_name: String) {
    let mut port = serialport::new(port_name, SERIAL_BAUDRATE)
                               .timeout(Duration::from_secs(60))
                               .open()
                               .expect("Unable to open port");
    loop {
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match port.read(serial_buf.as_mut_slice()) {
            Ok (_) => {
                match str::from_utf8(&serial_buf) {
                    Ok(message) => print!("{}", message),
                    Err(_) => println!("Error parsing serial input"),
                }
            },
            Err (_) => (),
        };

    }
}


pub fn position_updater(port_name: String, pos_channel: Receiver<RawPoint>) {
    let mut port = serialport::new(port_name, SERIAL_BAUDRATE)
                               .timeout(Duration::from_secs(1))
                               .open()
                               .expect("Unable to open port");

    let update_millis = 25;
    loop {
        let point = pos_channel.recv().expect("Error reading from channel");
        //println!("{:?}", point);

        if !point.is_valid() {
            println!("Point {:?} is not valid!", point);
            continue;
        }

        let output_bytes = point.get_bytes();

        port.write(&output_bytes).expect("write failed");
        thread::sleep(Duration::from_millis(update_millis));
    }

}

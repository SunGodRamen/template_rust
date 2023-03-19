extern crate hidapi;

use hidapi::{HidApi, HidDevice};
use std::time::Duration;
use std::thread;

const VENDOR_ID: u16 = 0x4000;
const PRODUCT_ID: u16 = 0x1111;

fn main() {
    let api = HidApi::new().expect("Failed to create a HidApi instance");

    let device = api
        .device_list()
        .find(|info| {
            info.vendor_id() == VENDOR_ID && info.product_id() == PRODUCT_ID
        })
        .and_then(|info| info.open_device(&api).ok())
        .expect("Failed to open the HID device");

    println!("Connected to the QMK keyboard!");

    loop {
        read_hid_messages(&device);
        write_hid_messages(&device);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn read_hid_messages(device: &HidDevice) {
    let mut buf = [0u8; 64];
    match device.read_timeout(&mut buf, 1000) {
        Ok(len) if len > 0 => {
            println!("Received HID message: {:?}", &buf[..len]);
        }
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error reading from the device: {:?}", e);
        }
    }
}

fn write_hid_messages(device: &HidDevice) {
    let msg = [0u8; 64]; // Replace with the HID message you want to send

    match device.write(&msg) {
        Ok(len) if len == msg.len() => {
            println!("Sent HID message: {:?}", &msg);
        }
        Ok(_) => {
            eprintln!("Failed to send the complete HID message");
        }
        Err(e) => {
            eprintln!("Error writing to the device: {:?}", e);
        }
    }
}

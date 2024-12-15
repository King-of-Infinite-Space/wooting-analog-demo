extern crate wooting_analog_wrapper;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

use anyhow::{Context, Result};
use log::*;
use simplelog::*;
use std::collections::HashMap;
use std::time::Duration;
use wooting_analog_wrapper as sdk;

const DEVICE_BUFFER_MAX: usize = 5;
const ANALOG_BUFFER_READ_MAX: usize = 40;

fn main() {
    // Initialize logging
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    if let Err(e) = run() {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    info!("Starting Wooting Analog SDK!");

    // Initialize the SDK
    let init_result = sdk::initialise();
    let device_num = match init_result.0 {
        Ok(device_num) => {
            info!(
                "Analog SDK Successfully initialised with {} devices",
                device_num
            );
            let devices = sdk::get_connected_devices_info(DEVICE_BUFFER_MAX).0?;
            for (i, device) in devices.iter().enumerate() {
                // Correct field names used here
                println!("Device {}: VID: {:04X}, PID: {:04X}, Manufacturer: {}, Name: {}, ID: {:?}, Type: {:?}", 
                         i, 
                         device.vendor_id, 
                         device.product_id, 
                         device.manufacturer_name, 
                         device.device_name,
                         device.device_id,
                         device.device_type);
            }
            device_num
        }
        Err(e) => {
            error!("Error initializing SDK: {}", e);
            return Err(e).context("Wooting Analog SDK Failed to initialise");
        }
    };

    // Check if any devices were found
    if device_num == 0 {
        warn!("No Wooting devices found.");
        return Ok(());
    }

    println!("Press any key on the Wooting keyboard to see the analog value. Press Ctrl+C to exit.");

    // Main loop to read analog data
    loop {
        // Read the full analog buffer
        let read_result = sdk::read_full_buffer(ANALOG_BUFFER_READ_MAX);
        match read_result.0 {
            Ok(analog_data) => {
                // Iterate through the received data
                for (key_id, value) in analog_data.iter() {
                    println!("Key ID: {}, Analog Value: {:.4}", key_id, value);
                }
            }
            Err(e) => {
                error!("Error reading analog buffer: {}", e);
            }
        }

        // Sleep for a short duration to avoid excessive CPU usage
        std::thread::sleep(Duration::from_millis(100));
    }
}
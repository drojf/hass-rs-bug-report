use std::{thread, time::Duration};

use async_std::task;
use hass_rs::client;

fn main() {
    let handle = thread::spawn(move || {
        task::block_on(async {
            println!("Running async task");
            let client = client::connect("PUT_HOST_NAME_HERE", 8123).await;

            // Removing this line causes a panic, keeping this line causes high CPU usage
            thread::sleep(Duration::from_secs(2));

            if let Err(e) = client {
                println!("Error during client::connect(): {:?}", e);
            }
            println!("Finished running async task with no errors");
        })
    });

    if let Err(e) = handle.join() {
        println!("Thread exited with error {:?}", e)
    }

    // Give some time for the panic to be reported and/or for you to view high CPU usage after task finishes
    thread::sleep(Duration::from_secs(15));
}

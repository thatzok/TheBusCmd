mod client;

use crate::client::get_current_vehicle;
use client::{RequestConfig, get_telemetry_data, send_telemetry_bus_cmd};

#[tokio::main]
async fn main() {
    let cmd = std::env::args().nth(1);
    if cmd.is_none() {
        println!("Usage: TheBusCmd <cmd>");
        return;
    }
    let cmd = cmd.unwrap();

    let vehicle = get_current_vehicle().await;

    if vehicle.is_empty() {
        println!("No vehicle found, not in bus.");
        return;
    }
    println!("Vehicle: {}", vehicle);

    let config = RequestConfig::new().vehicle(vehicle);

    let _ = send_telemetry_bus_cmd(&config, &cmd).await;

}

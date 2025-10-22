use the_bus_telemetry::api::get_current_vehicle_name;
use the_bus_telemetry::api::{RequestConfig, send_telemetry_bus_cmd};

#[tokio::main]
async fn main() {
    let cmd = std::env::args().nth(1);
    if cmd.is_none() {
        println!("Usage: TheBusCmd <cmd>");
        return;
    }
    let cmd = cmd.unwrap();
    let mut config = RequestConfig::new();

    let vehicle_name = get_current_vehicle_name(&config).await;

    if vehicle_name.is_empty() {
        println!("No vehicle found, not in bus.");
        return;
    }

    println!("Vehicle-Name: {}", vehicle_name);
    config.vehicle_name = vehicle_name;

    let _ = send_telemetry_bus_cmd(&config, &cmd).await;
}

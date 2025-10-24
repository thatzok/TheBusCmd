use std::time::Duration;
use the_bus_telemetry::api::{get_current_vehicle_name, get_vehicle, RequestConfig};
use the_bus_telemetry::api2vehicle::get_vehicle_state_from_api;
use the_bus_telemetry::vehicle::{init_vehicle_state, print_vehicle_state};
use the_bus_telemetry::vehicle_diff::compare_vehicle_states;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let sleeptime_error = 1000;
    let sleeptime_ok = 300;

    let interval_error = Duration::from_millis(sleeptime_error);
    let interval_ok = Duration::from_millis(sleeptime_ok);

    let mut vehicle_name = "".to_string();
    let mut old_vehicle_name = "".to_string();

    let mut config = RequestConfig::new();
    // config.debugging=true;

    let mut vehicle_state = init_vehicle_state();

    let mut zaehler = 0;

    loop {
        if (vehicle_name.is_empty()) || (zaehler > 10) {
            config.vehicle_name = "Current".to_string();
            vehicle_name = get_current_vehicle_name(&config).await;
            zaehler = 0;
        }

        if vehicle_name.is_empty() {
            println!("No vehicle found, not in bus.");
            vehicle_state = init_vehicle_state();
            old_vehicle_name = "".to_string();
            sleep(interval_error).await;
            continue;
        }

        if config.debugging {
            println!("Vehicle-Name: {}", vehicle_name);
        }

        config.vehicle_name = vehicle_name.clone();

        let vehicle_response = get_vehicle(&config).await;
        if vehicle_response.is_err() {
            println!("Error getting vehicle data in JSON.");
            vehicle_name = "".to_string();
            sleep(interval_error).await;
            continue;
        }

        zaehler += 1;
        let vehicle = vehicle_response.unwrap();
        if config.vehicle_model != vehicle.vehicle_model {
            config.vehicle_model = vehicle.vehicle_model.clone();
        }

        if vehicle_name != old_vehicle_name {
            println!(
                "Vehicle is now: model={} name={}",
                config.vehicle_model, vehicle_name
            );
            old_vehicle_name = vehicle_name.clone();
        }

        let new_vehicle_state = get_vehicle_state_from_api(vehicle);
        if config.debugging {
            print_vehicle_state(&new_vehicle_state);
        }

        compare_vehicle_states(&vehicle_state, &new_vehicle_state, false);

        vehicle_state = new_vehicle_state;

        sleep(interval_ok).await;
    }
}

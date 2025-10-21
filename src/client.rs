use std::time::Duration;

pub struct RequestConfig {
    host: String,
    port: String,
    vehicle: String,
    timeout: Duration,
}

impl RequestConfig {
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: "37337".to_string(),
            vehicle: "Current".to_string(),
            timeout: Duration::from_millis(300),
        }
    }
    pub fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn port(mut self, port: String) -> Self {
        self.port = port;
        self
    }
    pub fn vehicle(mut self, vehicle: String) -> Self {
        self.vehicle = vehicle;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

pub async fn send_telemetry_bus_cmd(
    config: &RequestConfig,
    cmd: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "http://{}:{}/vehicles/{}/{}",
        config.host, config.port, config.vehicle, cmd
    );
    println!("URL: {}", url);

    let response = reqwest::Client::new()
        .get(url)
        .timeout(config.timeout)
        .send()
        .await;

    Ok(())
}

pub async fn get_telemetry_data(
    config: &RequestConfig,
    path: &str,
) -> reqwest::Result<serde_json::Value> {
    let url = format!("http://{}:{}/{}", config.host, config.port, path);
    println!("URL: {}", url);

    let value = reqwest::Client::new()
        .get(url)
        .timeout(config.timeout)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(value)
}

pub async fn get_current_vehicle() -> String {
    let config = RequestConfig::new();

    // let data = get_telemetry_data(&config, "Vehicles").await;
    // if data.is_err() {
    //    return "".to_string();
    // }

    // let js = data.unwrap();

    // if let Some(vehicle) = js.get(0) {
    //    if let Some(vehicle_str) = vehicle.as_str() {
    //        return vehicle_str.to_string();
    //    }
    // }

    let result = get_telemetry_data(&config, "player").await;

    if result.is_err() {
        return "".to_string();
    }
    let data = result.unwrap();

    let mode: Option<String> = data
        .get("Mode")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut mo = "".to_string();
    if let Some(ref m) = mode {
        mo = m.clone();
    }

    if mo != "Vehicle" {
        return "".to_string();
    }

    let current_vehicle: Option<String> = data
        .get("CurrentVehicle")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut bus = "".to_string();

    if let Some(ref cv) = current_vehicle {
        bus = cv.clone();
    }
    bus
}

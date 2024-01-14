use rand::Rng;
use serde_json::json;
use std::time::{Duration, Instant};
use std::thread;
use tungstenite::{connect, Message};
use url::Url;

pub struct Device {
    type_str: String,
    interval: Duration,
    min_dp_1: f32,
    max_dp_1: f32,
    min_dp_2: f32,
    max_dp_2: f32,
    single_dp: bool,
    decreasing: bool,
    current_value: f32,
    ws_url: String,
}

impl Device {
    pub fn new(type_str: &str, interval_secs: u64, min_dp_1: f32, max_dp_1: f32, min_dp_2: f32, max_dp_2: f32, single_dp: bool, decreasing: bool) -> Self {
        Device {
            type_str: type_str.to_string(),
            interval: Duration::from_secs(interval_secs),
            min_dp_1,
            max_dp_1,
            min_dp_2,
            max_dp_2,
            single_dp,
            decreasing,
            current_value: 100.0,
            ws_url: format!("ws://127.0.0.1:8080/ws/?role=writer&topic={}", type_str),
        }
    }

    pub fn start(&self) {
        let ws_url = self.ws_url.clone();
        let type_str = self.type_str.to_string();
        let interval = self.interval;
        let min_dp_1 = self.min_dp_1;
        let max_dp_1 = self.max_dp_1;
        let min_dp_2 = self.min_dp_2;
        let max_dp_2 = self.max_dp_2;
        let single_dp =self.single_dp;
        let decreasing = self.decreasing;
        let mut current_value = self.current_value;

        thread::spawn(move || {
            let (mut socket, _response) = connect(Url::parse(&ws_url).unwrap()).expect("Can't connect");

            println!("Connected {} to the server", type_str);
            println!("Transmitting data every {:?}\n", interval);

            let mut instant = Instant::now();

            loop {
                
                if instant.elapsed() >= interval {
                    if decreasing {
                        let data = generate_decreasing_data(&type_str, &mut current_value, min_dp_1, max_dp_1);
                        socket.send(Message::Text(data)).unwrap();
                        instant = Instant::now();
                    }
                    else if single_dp {
                        let data = generate_single_data_point(&type_str, min_dp_1, max_dp_1);
                        socket.send(Message::Text(data)).unwrap();
                        instant = Instant::now();
                    }
                    else {
                        let data = generate_data(&type_str, min_dp_1, max_dp_1, min_dp_2, max_dp_2);
                        socket.send(Message::Text(data)).unwrap();
                        instant = Instant::now();
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

fn generate_data(device: &str, min_dp_1: f32, max_dp_1: f32, min_dp_2: f32, max_dp_2: f32) -> String {
    let mut rng = rand::thread_rng();
    let lat = rng.gen_range(min_dp_1..max_dp_1);
    let long = rng.gen_range(min_dp_2..max_dp_2);
    json!({ "deviceID": device.to_string(), "lat": lat, "long": long }).to_string()
}

fn generate_single_data_point(device: &str, min: f32, max: f32) -> String {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(min..max);
    json!({ "deviceID": device.to_string(), "value": value }).to_string()
}

fn generate_decreasing_data(device: &str, value: &mut f32, min: f32, max: f32) -> String {
    if *value < 0.0 {
        *value = 100.0;
        return json!({ "deviceID": device.to_string(), "value": "0.0" }).to_string();
    }
    let mut rng = rand::thread_rng();
    let sub_value = rng.gen_range(min..max);
    *value = *value - sub_value; 
    json!({ "deviceID": device.to_string(), "value": value }).to_string()
}
use rand::Rng;
use serde_json::json;
use std::time::{Duration, Instant};
use std::thread;
use tungstenite::{connect, Message};
use url::Url;

pub struct Device {
    type_str: String,
    interval: Duration,
    min_dp_1: f64,
    max_dp_1: f64,
    min_dp_2: f64,
    max_dp_2: f64,
    single_dp: bool,
    ws_url: String,
}

impl Device {
    pub fn new(type_str: &str, interval_secs: u64, min_dp_1: f64, max_dp_1: f64, min_dp_2: f64, max_dp_2: f64, single_dp: bool) -> Self {
        Device {
            type_str: type_str.to_string(),
            interval: Duration::from_secs(interval_secs),
            min_dp_1,
            max_dp_1,
            min_dp_2,
            max_dp_2,
            single_dp,
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

        thread::spawn(move || {
            let (mut socket, response) = connect(Url::parse(&ws_url).unwrap()).expect("Can't connect");

            println!("Connected to the server");
            println!("Response HTTP code: {}", response.status());
            println!("Response contains the following headers:");
            for (ref header, _value) in response.headers() {
                println!("* {}", header);
            }

            let mut instant = Instant::now();

            loop {
                if single_dp {
                    let data = generate_single_data_point(&type_str, min_dp_1, max_dp_1);
                    socket.write_message(Message::Text(data)).unwrap();
                    instant = Instant::now();
                }
                else if instant.elapsed() >= interval {
                    let data = generate_data(&type_str, min_dp_1, max_dp_1, min_dp_2, max_dp_2);
                    socket.write_message(Message::Text(data)).unwrap();
                    instant = Instant::now();
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}

fn generate_data(device: &str, min_dp_1: f64, max_dp_1: f64, min_dp_2: f64, max_dp_2: f64) -> String {
    let mut rng = rand::thread_rng();
    let lat = rng.gen_range(min_dp_1..max_dp_1);
    let long = rng.gen_range(min_dp_2..max_dp_2);
    json!({ "deviceID": device.to_string(), "lat": lat, "long": long }).to_string()
}

fn generate_single_data_point(device: &str, min: f64, max: f64) -> String {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(min..max);
    json!({ "deviceID": device.to_string(), "value": value }).to_string()
}

use actix::{Actor, StreamHandler, AsyncContext};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::str;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Define your WebSocket actor
struct TerraSocket {
    role: String,
    topic: Option<String>,
    topics_data: HashMap<String, (String, Instant)>,
}

impl Actor for TerraSocket {
    type Context = ws::WebsocketContext<Self>;

    // Actor logic here
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text(format!("{}", self.role));
        if self.role == "reader" {
            if let Some(topic_name) = self.topic.clone() {
                // Schedule a task to send random data periodically
                ctx.run_interval(Duration::from_secs(5), move |act, ctx| {
                    if let Some((value, _timestamp)) = act.topics_data.get(&topic_name) {
                        ctx.text(format!("Latest data for {}: {}", topic_name, value));
                    } else {
                        ctx.text(format!("NO DATA YET FOR {}", topic_name));
                    }
                });
            } else { ctx.text(format!("NO TOPIC YET")); }
        }
    }
}

// WebSocket StreamHandler for message handling
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TerraSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if self.role == "writer" {
                    // Update topic data
                    if let Some(topic_name) = &self.topic {
                        self.topics_data.insert(topic_name.clone(), (text.to_string(), Instant::now()));
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                if let Ok(text) = str::from_utf8(&bin) {
                    println!("Received binary message: {}", text);
                    // Handle binary message as text
                }
            }
            _ => {
                // Handle other message types like Ping, Pong, Close
            }
        }
    }
}

// Setup WebSocket route
async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let query_string = req.query_string();
    let query_params: HashMap<String, String> = url::form_urlencoded::parse(query_string.as_bytes())
        .into_owned()
        .collect();

    let role = query_params.get("role").unwrap_or(&String::from("default_role")).to_string();
    let topic = query_params.get("topic").map(|t| t.to_string());

    let socket = TerraSocket { 
        role, 
        topic,
        topics_data: HashMap::new(),
    };
    
    ws::start(socket, &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/ws/", web::get().to(websocket_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

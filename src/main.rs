use actix::{Actor, StreamHandler, AsyncContext};
use actix_web::error::ErrorInternalServerError;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::str;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use log::{error, info};
use std::sync::{Mutex, Arc};

struct AppState {
    topics_data: Mutex<HashMap<String, (String, Instant)>>,
}

// Define your WebSocket actor
struct TerraSocket {
    role: String,
    topic: Option<String>,
    state: Arc<AppState>,
}

impl Actor for TerraSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection started. Role: {}, Topic: {:?}", self.role, self.topic);
        if self.role == "reader" {
            if let Some(topic_name) = self.topic.clone() {
                // Schedule a task to send data periodically
                let state = self.state.clone();
                ctx.run_interval(Duration::from_secs(5), move |_, ctx| {
                    let topics_data = state.topics_data.lock().unwrap(); // Lock the mutex
                    if let Some((value, _timestamp)) = topics_data.get(&topic_name) {
                        ctx.text(format!("Latest data for {}: {}", topic_name, value));
                    } else {
                        ctx.text(format!("NO DATA YET FOR {}", topic_name));
                    }
                });
            } else {
                ctx.text("No topic specified for reader.");
            }
        }
    }
}


// WebSocket StreamHandler for message handling
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TerraSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if self.role == "writer" {
                    if let Some(topic_name) = &self.topic {
                        let mut topics_data = self.state.topics_data.lock().unwrap(); // Lock the mutex
                        topics_data.insert(topic_name.clone(), (text.to_string(), Instant::now()));
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
async fn websocket_route(req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Arc<AppState>>) -> Result<HttpResponse, Error> {
    let query_string = req.query_string();
    info!("Raw query string: {}", query_string);

    let query_params: HashMap<String, String> = url::form_urlencoded::parse(query_string.as_bytes())
        .into_owned()
        .collect();

    info!("Query params parsed");

    let role = query_params.get("role").unwrap_or(&String::from("default_role")).to_string();
    let topic = query_params.get("topic").map(|t| t.to_string());

    info!("WebSocket connection requested. Role: {}, Topic: {:?}", role, topic);

    let socket = TerraSocket { 
        role, 
        topic,
        state: data.get_ref().clone(),
    }; 
    
    ws::start(socket, &req, stream).map_err(|e| {
        error!("WebSocket error occurred: {}", e);
        ErrorInternalServerError("Error starting WebSocket session")
    })
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let shared_state = Arc::new(AppState {
        topics_data: Mutex::new(HashMap::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_state.clone()))
            .route("/ws/", web::get().to(websocket_route))
    })
    .bind("127.0.0.1:8080")
    .map_err(|err| {
        error!("Failed to bind to address: {}", err);
        err
    })?
    .run()
    .await
}

use actix::{Actor, StreamHandler, AsyncContext};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::str;
use rand::Rng; // For generating random numbers
use std::time::Duration;

// Define your WebSocket actor
struct TerraSocket;

impl Actor for TerraSocket {
    type Context = ws::WebsocketContext<Self>;

    // Actor logic here
    fn started(&mut self, ctx: &mut Self::Context) {
        // Schedule a task to send random data periodically
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            // Generate random data
            let random_data = rand::thread_rng().gen_range(0..100);
            // Send the random data to the client
            ctx.text(format!("Periodic random data: {}", random_data));
        });
    }
}

// WebSocket StreamHandler for message handling
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TerraSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received text message: {}", text);
                // Handle text message
            
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
    ws::start(TerraSocket, &req, stream)
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

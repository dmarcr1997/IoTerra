use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

//Defined your WebSocket actor
struct TerraSocket;

impl Actor for TerraSocket {
    type Context = ws::WebsocketContext<Self>;

    //Actor logic below
}

//Websocket StreamHandler for message handling
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TerraSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        //Handle messages here
    }
}

//Setup WebSocket route
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

// Unit tests
#[cfg(test)]
mod server_tests {
    use super::*;
    use actix_web::{test, web, App};

    // Test the WebSocket route
    #[actix_rt::test]
    async fn test_websocket_route() {
        let mut app = test::init_service(
            App::new().route("/ws/", web::get().to(websocket_route))
        ).await;

        // Create a WebSocket upgrade request
        let req = test::TestRequest::with_uri("/ws/")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Key", "test")
            .header("Sec-WebSocket-Version", "13")
            .to_request();

        // Simulate a WebSocket connection
        let (resp, mut connection) = test::start_ws(&mut app, req)
            .await
            .expect("Failed to start WebSocket connection");

        //should return a 101 status for switching protocols
        assert_eq!(resp.status(), 101);
    }
}
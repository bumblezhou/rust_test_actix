use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

pub struct WebSocketActor {
    // Define any state you need here
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Handle incoming text message
                println!("Received message: {}", text);
                // Example: echo back to client
                ctx.text(text);
            }
            _ => (),
        }
    }
}
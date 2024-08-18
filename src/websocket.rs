use actix::{Actor, StreamHandler, Addr, Handler, AsyncContext};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

pub struct GridWebSocket {
    pub clients: Arc<Mutex<HashSet<Addr<GridWebSocket>>>>,
}

impl Actor for GridWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().insert(addr);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.clients.lock().unwrap().remove(&addr);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GridWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => (),
        }
    }
}

pub struct GridUpdate(pub String);

impl Handler<GridUpdate> for GridWebSocket {
    type Result = ();

    fn handle(&mut self, msg: GridUpdate, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl actix::Message for GridUpdate {
    type Result = ();
}
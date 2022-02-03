use actix::prelude::*;
use actix_web_actors::ws;
use log::{debug, info, warn};

use crate::{
    message::{GetRoom, JoinRoom, WsMessage},
    server::WsQuizServer,
};

#[derive(Default)]
pub(crate) struct WsSession {
    id: usize,
    room: String,
    name: Option<String>,
}

impl WsSession {
    fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = room_name.to_string();

        // TODO ルーム退室処理

        todo!()
        //     let room_addr = {
        //         let get_room_msg = GetRoom {
        //             room_name: room_name.to_owned(),
        //         };
        //         WsQuizServer::from_registry()
        //             .send(get_room_msg)
        //             .into_actor(self)
        //             .then(|addr, act, _ctx| {
        //                 if let Ok(room_addr) = addr {
        //                     let join_room_msg = JoinRoom {
        //                         room_name: room_name.to_owned(),
        //                         name: self.name.clone(),
        //                         addr: ctx.address().recipient()
        //                     }
        //                     room_addr.send(join_room_msg).into_actor(self).then(|a,s,f|
        //                     {
        //                         todo!()
        //                 fut::ready(())

        //                     })
        //                 }
        //                 fut::ready(())
        //             })
        //     }
        //     .wait(ctx);
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsSession connected");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!(
            "WsSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            self.id,
            self.room
        );
    }
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        debug!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();

                if msg.starts_with('/') {
                    let mut command = msg.splitn(2, ' ');

                    match command.next() {
                        Some("/join") => if let Some(room_name) = command.next() {},
                        _ => warn!("Unknown command: {:?}", msg),
                    }
                }
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

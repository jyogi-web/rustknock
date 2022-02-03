use actix::prelude::*;
use actix_web_actors::ws;
use log::{debug, info, warn};

use crate::{
    message::{GetRoom, JoinRoom, LeaveRoom, WsMessage},
    room::QuizRoom,
    server::WsQuizServer,
};

#[derive(Default)]
pub struct WsSession {
    id: usize,
    room: String,
    name: Option<String>,
    room_addr: Option<Addr<QuizRoom>>,
}

impl WsSession {
    fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = room_name.to_owned();

        // doneTODO ルーム退室処理
        if let Some(room_addr) = self.room_addr.as_mut() {
            let leave_msg = LeaveRoom { id: self.id };

            room_addr.do_send(leave_msg);
            self.room_addr = None;
        }

        let room_addr = {
            let get_room_msg = GetRoom {
                room_name: room_name.to_owned(),
            };

            WsQuizServer::from_registry()
                // ルーム情報取得
                .send(get_room_msg)
                .into_actor(self)
                .then(|addr, act, ctx| {
                    if let Ok(room_addr) = addr {
                        let join_room_msg = JoinRoom {
                            name: act.name.clone(),
                            addr: ctx.address().recipient(),
                        };

                        room_addr
                            // ルーム加入リクエスト
                            .send(join_room_msg)
                            .into_actor(act)
                            .then(|res, act, ctx| {
                                match res {
                                    Ok(Ok((id, addr))) => {
                                        act.id = id;
                                        act.room = room_name;
                                        act.room_addr = Some(addr);
                                    }
                                    Ok(Err(err_msg)) => ctx.text(err_msg),
                                    _ => (),
                                }

                                fut::ready(())
                            })
                            .wait(ctx);
                    }

                    fut::ready(())
                })
                .wait(ctx);
        };
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsSession connected");
        // self.join_room("Main", ctx);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!(
            "WsSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            self.id,
            self.room
        );

        if let Some(room_addr) = self.room_addr.as_mut() {
            let leave_msg = LeaveRoom { id: self.id };

            room_addr.do_send(leave_msg);
            self.room_addr = None;
        }
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
                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                self.join_room(room_name, ctx)
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
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

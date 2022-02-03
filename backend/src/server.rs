use std::collections::HashMap;

use actix::prelude::*;
use log::info;

use crate::{
    message::{GetRoom, JoinRoom},
    room::{self, QuizRoom, User},
};

type QuizRoomAddr = Addr<QuizRoom>;

#[derive(Default)]
pub(crate) struct WsQuizServer {
    rooms: HashMap<String, QuizRoomAddr>,
}

impl Actor for WsQuizServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsQuizServer started");
    }
}

impl WsQuizServer {
    // fn add_client_to_room(&mut self, room_name: &str, id: Option<usize>) -> usize {
    //     let mut id = id.unwrap_or_else(rand::random);

    //     if let Some(room) = self.rooms.get_mut(room_name) {
    //         loop {
    //             if room.users().contains_key(&id) {
    //                 id = rand::random::<usize>();
    //             } else {
    //                 break;
    //             }
    //         }

    //         room.add_user(id, User::default());
    //         return id;
    //     }

    //     let mut room = QuizRoom::default();
    //     room.start();
    //     room.add_user(id, User::default());

    //     self.rooms.insert(room_name.to_string(), room);

    //     id
    // }
}

impl Handler<GetRoom> for WsQuizServer {
    type Result = MessageResult<GetRoom>;

    fn handle(&mut self, msg: GetRoom, ctx: &mut Self::Context) -> Self::Result {
        let GetRoom { room_name } = msg;

        if let Some(room_addr) = self.rooms.get(&room_name) {
            return MessageResult(room_addr.clone());
        }

        let room = QuizRoom::new(&room_name);
        let room_addr = room.start();

        self.rooms.insert(room_name, room_addr.clone());

        MessageResult(room_addr)
    }
}

impl SystemService for WsQuizServer {}
impl Supervised for WsQuizServer {}

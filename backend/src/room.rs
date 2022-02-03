use std::collections::HashMap;

use actix::prelude::*;
use getset::Getters;
use log::info;
use quiz_json::Quiz;

use crate::message::{JoinRoom, LeaveRoom};

#[derive(Default, Getters)]
pub(crate) struct User {
    name: String,
    score: usize,
}

impl User {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        Self {
            name,
            ..Default::default()
        }
    }
}

enum QuizLifecycle {
    Ready,
    Starting,
    Started,
    DuringAnswer,
    Stopping,
    Stopped,
}

impl Default for QuizLifecycle {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Default, Getters)]
pub(crate) struct QuizRoom {
    room_name: String,
    #[getset(get = "pub")]
    users: HashMap<usize, User>,
    state: QuizLifecycle,
}

impl QuizRoom {
    pub(crate) fn new(room_name: &str) -> Self {
        let room_name = room_name.to_string();
        Self {
            room_name,
            ..Default::default()
        }
    }

    pub(crate) fn add_user(&mut self, id: usize, user: User) {
        self.users.insert(id, user);
    }
}

impl Actor for QuizRoom {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("New quiz room {} started", &self.room_name);
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Quiz room {} stopped", &self.room_name);
    }
}

impl Handler<JoinRoom> for QuizRoom {
    type Result = MessageResult<JoinRoom>;

    fn handle(&mut self, msg: JoinRoom, ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom { name, addr } = msg;
        let mut id = rand::random::<usize>();

        loop {
            if self.users.contains_key(&id) {
                id = rand::random::<usize>();
            } else {
                break;
            }
        }

        let user = User::new(&name.unwrap_or_else(|| "anonymous".to_string()));
        self.users.insert(id, user);

        MessageResult((id, ctx.address()))
    }
}

impl Handler<LeaveRoom> for QuizRoom {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) -> Self::Result {
        let LeaveRoom { id } = msg;

        if let Some(_) = self.users.remove(&id) {
            info!("Leave room: {} id: {}", &self.room_name, &id);
        }
    }
}

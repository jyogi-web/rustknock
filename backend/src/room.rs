use std::collections::HashMap;

use actix::prelude::*;
use getset::Getters;
use log::{debug, info};
use quiz_json::Quiz;

use crate::message::{JoinRoom, LeaveRoom, QuizStartRequest, WsMessage};

const QUIZ_QUESTION_NUMBER: usize = 5;
const select_quizzes_endpoint: &'static str = "localhost:3000/quiz/";

#[derive(Getters)]
pub(crate) struct User {
    name: String,
    score: usize,
    addr: Recipient<WsMessage>,
}

impl User {
    fn new(name: &str, addr: Recipient<WsMessage>) -> Self {
        let name = name.to_string();
        Self {
            name,
            score: 0,
            addr,
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
    quizzes: Vec<Quiz>,
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
        // readyでなければ加入を弾く
        if let QuizLifecycle::Ready = self.state {
        } else {
            return MessageResult(Err(format!(
                "!!! The quiz has already started in {} room",
                self.room_name
            )));
        }

        let JoinRoom { name, addr } = msg;
        let mut id = rand::random::<usize>();

        loop {
            if self.users.contains_key(&id) {
                id = rand::random::<usize>();
            } else {
                break;
            }
        }

        let user = User::new(&name.unwrap_or_else(|| "anonymous".to_string()), addr);
        self.users.insert(id, user);

        MessageResult(Ok((id, ctx.address())))
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

impl Handler<QuizStartRequest> for QuizRoom {
    type Result = ();

    fn handle(&mut self, _msg: QuizStartRequest, ctx: &mut Self::Context) -> Self::Result {
        if let QuizLifecycle::Ready = self.state {
            let res = reqwest::blocking::get(format!(
                "http://{}{}",
                select_quizzes_endpoint, QUIZ_QUESTION_NUMBER
            ));
            if let Ok(res) = res {
                self.quizzes = if let Ok(json) = res.json() {
                    json
                } else {
                    return;
                };
                debug!("{:?}", &self.quizzes);
            }

            info!("Start quiz in {} room", &self.room_name);
            self.state = QuizLifecycle::Starting;
        }
    }
}

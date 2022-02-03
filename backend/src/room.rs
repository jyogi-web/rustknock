use std::{collections::HashMap, sync::Arc, time::Duration};

use actix::prelude::*;
use actix_daemon_utils::{
    delayer::{Delayer, Task, Timing},
    graceful_stop::GracefulStop,
};
use actix_web::client;
use getset::Getters;
use log::{debug, info, warn};
use quiz_json::Quiz;

use crate::message::{DeleteUser, JoinRoom, LeaveRoom, QuizStartRequest, WsMessage};

const QUIZ_QUESTION_NUMBER: usize = 5;
const SELECT_QUIZZES_ENDPOINT: &'static str = "localhost:3000/quiz/";
const DELAY_START: u64 = 5000;
const QUIZ_LIMIT_TIME: u64 = 30;

#[derive(Getters, Debug, Clone)]
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
    AnswerWaiting,
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

type Users = HashMap<usize, User>;

#[derive(Default, Getters)]
pub(crate) struct QuizRoom {
    room_name: String,
    #[getset(get = "pub")]
    users: Users,
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

    fn take_user(&mut self) -> Option<Users> {
        let users = std::mem::take(&mut self.users);

        Some(users)
    }

    fn broadcast_message(&mut self, msg: &str) -> Option<()> {
        let mut users = self.take_user()?;

        for (id, user) in users.drain() {
            if user.addr.do_send(WsMessage(msg.to_string())).is_ok() {
                self.add_user(id, user);
            }
        }

        Some(())
    }

    fn delay_send_quiz(&mut self, ms: u64, ctx: &mut Context<Self>) {
        let graceful_stop = GracefulStop::new();
        let delay_actor = DelayActor::new(
            &format!(
                "/question {} {}",
                QUIZ_LIMIT_TIME,
                self.quizzes.pop().unwrap_or_default().question()
            ),
            self.users.clone(),
            ctx.address().recipient(),
            DELAY_START,
        )
        .start();
        let delayer = Delayer::new(
            delay_actor.recipient(),
            graceful_stop.clone_system_terminator(),
            Duration::from_millis(500),
        )
        .start();

        graceful_stop.subscribe(delayer.recipient()).start();
    }
}

impl Actor for QuizRoom {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("New quiz {} room started", &self.room_name);
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Quiz {} room stopped", &self.room_name);
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
            info!("Leave {} room id: {}", &self.room_name, &id);
        }
    }
}

impl Handler<QuizStartRequest> for QuizRoom {
    type Result = ();

    fn handle(&mut self, _msg: QuizStartRequest, ctx: &mut Self::Context) -> Self::Result {
        if let QuizLifecycle::Ready = self.state {
            let res = reqwest::blocking::get(format!(
                "http://{}{}",
                SELECT_QUIZZES_ENDPOINT, QUIZ_QUESTION_NUMBER
            ));
            if let Ok(res) = res {
                self.quizzes = if let Ok(json) = res.json() {
                    json
                } else {
                    return;
                };
                debug!("{:?}", &self.quizzes);
            }

            self.broadcast_message(&format!("/started"));
            self.delay_send_quiz(DELAY_START, ctx);

            info!("Start quiz in {} room", &self.room_name);
            self.state = QuizLifecycle::Starting;
        }
    }
}

impl Handler<DeleteUser> for QuizRoom {
    type Result = ();

    fn handle(&mut self, msg: DeleteUser, ctx: &mut Self::Context) -> Self::Result {
        let DeleteUser(id) = msg;

        if self.users.contains_key(&id) {
            self.users.remove(&id).unwrap();
        }
    }
}

#[derive(Debug)]
pub struct DelayActor {
    msg: String,
    users: Users,
    room_addr: Recipient<DeleteUser>,
    delay_ms: u64,
    is_first_execution: bool,
}

impl DelayActor {
    fn new(msg: &str, users: Users, room_addr: Recipient<DeleteUser>, delay_ms: u64) -> Self {
        let msg = msg.to_string();
        Self {
            msg,
            users,
            room_addr,
            delay_ms,
            is_first_execution: true,
        }
    }

    fn broadcast_message(&mut self) -> Option<()> {
        for (id, user) in self.users.drain() {
            if user.addr.do_send(WsMessage(self.msg.to_string())).is_err() {
                match self.room_addr.do_send(DeleteUser(id.to_owned())) {
                    Ok(_) => (),
                    Err(err) => warn!("Error failed to do_send: {}", err),
                }
            }
        }

        Some(())
    }
}

impl Default for DelayActor {
    fn default() -> Self {
        Self {
            is_first_execution: true,
            ..Default::default()
        }
    }
}

impl Actor for DelayActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("DelayActor started {:?}", self);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("DelayActor stoped {:?}", self);
    }
}

impl Handler<Task> for DelayActor {
    type Result = ();

    fn handle(&mut self, msg: Task, ctx: &mut Self::Context) -> Self::Result {
        if self.is_first_execution {
            msg.0
                .do_send(Timing::Later(Duration::from_millis(self.delay_ms)));
            self.is_first_execution = false;
            debug!("first delay task");
        } else {
            self.broadcast_message();
            debug!("問題配信");
            ctx.stop();
        }
    }
}

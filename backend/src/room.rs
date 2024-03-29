use std::{collections::HashMap, ops::RangeBounds, sync::Arc, time::Duration};

use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_daemon_utils::{
    delayer::{Delayer, Task, Timing},
    graceful_stop::GracefulStop,
};
use actix_web::client;
use getset::Getters;
use log::{debug, info, warn};
use quiz_json::Quiz;
use serde::{Deserialize, Serialize};

use crate::message::{
    AnswerRequest, AnswerRightRequest, DelayNotification, DeleteUser, EntryName, JoinRoom,
    LeaveRoom, QuizStartRequest, StopDelayActor, StopQuizRoom, WsMessage,
};

const QUIZ_QUESTION_NUMBER: usize = 5;
const SELECT_QUIZZES_ENDPOINT: &'static str = "http://localhost:3000/quiz/";
const DELAY_START_MS: u64 = 3000;
const QUIZ_LIMIT_TIME_MS: u64 = 60000;
const INTERVAL_OF_QUIZ_MS: u64 = 5000;

#[derive(Getters, Debug, Clone)]
pub(crate) struct User {
    name: String,
    score: usize,
    addr: Recipient<WsMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserJson {
    pub id: usize,
    pub name: String,
    pub score: usize,
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

#[derive(Debug, Clone, PartialEq)]
enum QuizLifecycle {
    Ready,
    Started,
    AnswerRightWaiting,
    AnswerWaiting,
    Result,
    Stopping,
    Stopped,
}

impl Default for QuizLifecycle {
    fn default() -> Self {
        Self::Ready
    }
}

type Users = HashMap<usize, User>;

#[derive(Default, Getters, Debug)]
pub(crate) struct QuizRoom {
    room_name: String,
    #[getset(get = "pub")]
    users: Users,
    state: QuizLifecycle,
    quizzes: Vec<Quiz>,
    current_quiz_number: u32,
    current_quiz: Option<Quiz>,
    during_answer_id: Option<usize>,
    delay_actor_addr: Option<Addr<DelayActor>>,
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

    fn broadcast_message_with_filter(&mut self, msg: &str, filter_id: usize) -> Option<()> {
        let mut users = self.take_user()?;
        // BUG 送ってないユーザが消えてる

        for (id, user) in users.drain() {
            // WARN バグの暫定対応
            if id == filter_id {
                self.add_user(id, user);
                continue;
            }

            if id != filter_id && user.addr.do_send(WsMessage(msg.to_string())).is_ok() {
                self.add_user(id, user);
            }
        }

        Some(())
    }

    fn delay_notification(&mut self, ms: u64, ctx: &mut Context<Self>) {
        if let Some(addr) = self.delay_actor_addr.as_ref() {
            addr.do_send(StopDelayActor);
        }

        let graceful_stop = GracefulStop::new();
        let delay_actor = DelayActor::new(ctx.address(), ms).start();
        let delayer = Delayer::new(
            delay_actor.recipient(),
            graceful_stop.clone_system_terminator(),
            Duration::from_millis(500),
        )
        .start();

        graceful_stop.subscribe(delayer.recipient()).start();
    }

    fn send_users_data_json(&mut self) {
        let mut users = Vec::new();

        for (id, user) in &self.users {
            users.push(UserJson {
                id: id.to_owned(),
                name: user.name.to_owned(),
                score: user.score,
            });
        }

        self.broadcast_message(&format!(
            "/users {}",
            &serde_json::to_string(&users).unwrap()
        ));
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
        debug!("Join id {} in {} room", &id, self.room_name);
        self.add_user(id, user);
        self.send_users_data_json();

        MessageResult(Ok((id, ctx.address())))
    }
}

impl Handler<EntryName> for QuizRoom {
    type Result = ();

    fn handle(&mut self, msg: EntryName, _ctx: &mut Self::Context) -> Self::Result {
        let EntryName { id, name } = msg;

        if let Some(user) = self.users.get_mut(&id) {
            info!("Entry id {} user_name {}", &id, &name);
            user.name = name;
            self.send_users_data_json();
        }
    }
}

impl Handler<LeaveRoom> for QuizRoom {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) -> Self::Result {
        let LeaveRoom { id } = msg;

        if let Some(_) = self.users.remove(&id) {
            info!("Leave {} room id: {}", &self.room_name, &id);
            self.send_users_data_json();
            if self.state == QuizLifecycle::AnswerWaiting && self.during_answer_id == Some(id) {
                self.state = QuizLifecycle::AnswerRightWaiting;
                self.during_answer_id = None;
            }
        }
    }
}

impl Handler<QuizStartRequest> for QuizRoom {
    type Result = ();

    fn handle(&mut self, _msg: QuizStartRequest, ctx: &mut Self::Context) -> Self::Result {
        if let QuizLifecycle::Ready = self.state {
            // 問題をリクエスト
            // NOTE ローカルで実装しているAPIから取得しているがここだけ外部に移行してもいいかも
            let res = reqwest::blocking::get(format!(
                "{}{}",
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

            // クイズセクション開始を合図
            self.broadcast_message(&format!("/quiz_started"));
            self.delay_notification(DELAY_START_MS, ctx);

            info!("Start quiz in {} room", &self.room_name);
            self.state = QuizLifecycle::Started;
        }
    }
}

impl Handler<DeleteUser> for QuizRoom {
    type Result = ();

    fn handle(&mut self, msg: DeleteUser, ctx: &mut Self::Context) -> Self::Result {
        let DeleteUser(id) = msg;

        if self.users.contains_key(&id) {
            self.users.remove(&id).unwrap();
            debug!("delete user id {}", &id);
        }
    }
}

impl Handler<DelayNotification> for QuizRoom {
    type Result = ();

    fn handle(&mut self, msg: DelayNotification, ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            QuizLifecycle::Ready => (),
            QuizLifecycle::Started => {
                if let Some(question) = self.quizzes.pop() {
                    self.current_quiz_number += 1;
                    info!("{}問目配信", self.current_quiz_number);
                    self.broadcast_message(&format!(
                        "/question {} {}",
                        QUIZ_LIMIT_TIME_MS,
                        question.question()
                    ));
                    self.current_quiz = Some(question);
                    self.state = QuizLifecycle::AnswerRightWaiting;
                    self.delay_notification(QUIZ_LIMIT_TIME_MS, ctx);
                } else {
                    info!("結果発表にいくよ");
                    self.state = QuizLifecycle::Result;
                    self.delay_notification(INTERVAL_OF_QUIZ_MS, ctx);
                }
            }
            QuizLifecycle::AnswerRightWaiting | QuizLifecycle::AnswerWaiting => {
                info!("時間切れ");
                self.state = QuizLifecycle::Started;
                self.broadcast_message("/timeup");
                let (correct, explanatory) = if let Some(quiz) = self.current_quiz.as_ref() {
                    (
                        format!("/question_answer {}", quiz.answers()[0]),
                        format!("/explanatory {}", quiz.explanatory()),
                    )
                } else {
                    (String::new(), String::new())
                };
                self.broadcast_message(&correct);
                self.broadcast_message(&explanatory);
                self.delay_notification(INTERVAL_OF_QUIZ_MS, ctx);
            }
            QuizLifecycle::Result => {
                info!("けっかはっぴょぉおおおおおおおおおおおおお");
                self.send_users_data_json();
                self.broadcast_message("/result");

                info!("Stop QuizRoom");
                self.issue_system_async(StopQuizRoom {
                    room_name: self.room_name.to_owned(),
                });
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Handler<AnswerRightRequest> for QuizRoom {
    type Result = MessageResult<AnswerRightRequest>;

    fn handle(&mut self, msg: AnswerRightRequest, ctx: &mut Self::Context) -> Self::Result {
        let AnswerRightRequest(id) = msg;
        if let QuizLifecycle::AnswerRightWaiting = self.state {
            self.state = QuizLifecycle::AnswerWaiting;
            self.during_answer_id = Some(id);
            self.broadcast_message_with_filter("/ans_lock", id);

            return MessageResult(Ok(()));
        }

        MessageResult(Err("/ans_err".to_string()))
    }
}

impl Handler<AnswerRequest> for QuizRoom {
    type Result = MessageResult<AnswerRequest>;

    fn handle(&mut self, msg: AnswerRequest, ctx: &mut Self::Context) -> Self::Result {
        let AnswerRequest { id, answer } = msg;
        let mut is_correct_answer = false;

        if self.state == QuizLifecycle::AnswerWaiting
            && self.during_answer_id.unwrap_or_default() == id
        {
            if self
                .current_quiz
                .as_ref()
                .unwrap()
                .answers()
                .contains(&answer)
            {
                if let Some(user) = self.users.get_mut(&id) {
                    user.score += 1;
                    is_correct_answer = true;
                    info!("正解 id:{} score:{} ans:{}", id, user.score, answer);
                } else {
                    debug!("ここには入らないはず id {} users {:?}", id, self.users);
                }
                self.during_answer_id = None;
                self.state = QuizLifecycle::Started;
                self.broadcast_message_with_filter(
                    &format!("/others_correct_answer {} {}", &id, &answer),
                    id,
                );
                self.broadcast_message_with_filter("/ans_unlock", id);
                let explanatory = if let Some(quiz) = self.current_quiz.as_ref() {
                    format!("/explanatory {}", quiz.explanatory())
                } else {
                    String::new()
                };
                self.broadcast_message(&explanatory);
                self.delay_notification(INTERVAL_OF_QUIZ_MS, ctx);
            } else {
                self.during_answer_id = None;
                self.state = QuizLifecycle::AnswerRightWaiting;
                self.broadcast_message_with_filter("/ans_unlock", id);
                self.broadcast_message_with_filter(
                    &format!("/others_incorrect_answer {} {}", &id, &answer),
                    id,
                );
            }
        }

        if is_correct_answer {
            self.send_users_data_json();
            info!("Send users data");
            return MessageResult(Ok(()));
        } else {
            debug!("正解ではない");
        }

        MessageResult(Err("/incorrect".to_string()))
    }
}

#[derive(Debug)]
pub struct DelayActor {
    room_addr: Addr<QuizRoom>,
    delay_ms: u64,
    is_first_execution: bool,
}

impl DelayActor {
    fn new(room_addr: Addr<QuizRoom>, delay_ms: u64) -> Self {
        Self {
            room_addr,
            delay_ms,
            is_first_execution: true,
        }
    }

    // fn broadcast_message(&mut self) -> Option<()> {
    //     for (id, user) in self.users.drain() {
    //         if user.addr.do_send(WsMessage(self.msg.to_string())).is_err() {
    //             match self.room_addr.do_send(DeleteUser(id.to_owned())) {
    //                 Ok(_) => (),
    //                 Err(err) => warn!("Error failed to do_send: {}", err),
    //             }
    //         }
    //     }

    //     Some(())
    // }
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
            debug!("First delay task");
        } else {
            // self.broadcast_message();
            self.room_addr.do_send(DelayNotification);
            debug!("Send delay notification");
            ctx.stop();
        }
    }
}

impl Handler<StopDelayActor> for DelayActor {
    type Result = ();

    fn handle(&mut self, _msg: StopDelayActor, ctx: &mut Self::Context) -> Self::Result {
        info!("Stop DelayActor");
        ctx.stop();
    }
}

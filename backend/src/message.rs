use actix::prelude::*;

use crate::room::QuizRoom;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct WsMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "Result<(usize, Addr<QuizRoom>), String>")]
pub(crate) struct JoinRoom {
    pub name: Option<String>,
    pub addr: Recipient<WsMessage>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct LeaveRoom {
    pub id: usize,
}

#[derive(Clone, Message)]
#[rtype(result = "Addr<QuizRoom>")]
pub(crate) struct GetRoom {
    pub room_name: String,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct QuizStartRequest;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct DeleteUser(pub usize);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct DelayNotification;

#[derive(Clone, Message)]
#[rtype(result = "Result<(), String>")]
pub(crate) struct AnswerRightRequest(pub usize);

#[derive(Clone, Message)]
#[rtype(result = "Result<(), String>")]
pub(crate) struct AnswerRequest {
    pub id: usize,
    pub answer: String,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct StopDelayActor;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub(crate) struct StopQuizRoom {
    pub room_name: String,
}

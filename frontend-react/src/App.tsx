import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import Welcome from "./Welcome";
import Quiz from "./Quiz";
import { stringify } from "querystring";
import { type } from "os";
import Countdown from "react-countdown";
import useCountDown from "react-countdown-hook";
import { json } from "stream/consumers";
import Result from "./Result";
// import socketIOClient from "socket.io-client";

// const URL = "wss://rustknock-server.azurewebsites.net/ws/";
const URL = "ws://localhost:3000/ws/";
const webSocket = new WebSocket(URL);

export type User = {
  id: number;
  name: string;
  score: number;
};

export type Users = {
  userdata: User[];
};

interface Props {}

const App: React.FC<Props> = (props) => {
  const [isWelcome, setIsWelcome] = useState(true);
  const [currentQuestion, setCurrentQuestion] = useState("");
  const [isAnswerRight, setIsAnswerRight] = useState(false);
  const [isAnswerLock, setIsAnswerLock] = useState(false);
  // TODO timeup
  const [currentQuestionAnswer, setCurrentQuestionAnswer] = useState("");
  const [currentQuestionExplanatory, setCurrentQuestionExplanatory] =
    useState("");
  const [isTimeUp, setIsTimeUp] = useState(false);
  const [othersAnswer, setOthersAnswer] = useState("");
  const [userData, setUserData] = useState(
    (JSON.parse("[]") as Users).userdata
  );
  const [isStarted, setIsStarted] = useState(false);
  const [answerResult, setAnswerResult] = useState("");
  const [isMyAnswer, setIsMyAnswer] = useState("");
  const [isQuestion, setIsQuestion] = useState(true);
  const [timeLimitMs, setTimeLimit] = useState(0);
  const [timeLeftMs, { start, pause, resume, reset }] = useCountDown(0, 100);
  const [isEnd, setIsEnd] = useState(false);

  // TODO others_correct_answer
  // TODO others_incorrect_answer

  useEffect(() => {
    webSocket.onopen = () => {
      console.log("WebSocket Connected");
    };
    webSocket.onmessage = (data) => {
      console.log(data.data);
      // const msg: string = data.data;
      const split: string = data.data.split(" ");
      const command = split[0];

      if (command == "/quiz_started") {
        setIsStarted(true);
      } else if (command == "/question") {
        const tl = Number.parseInt(split[1]);
        const question = split[2];
        setCurrentQuestion(question);
        setCurrentQuestionAnswer("");
        setCurrentQuestionExplanatory("");
        setAnswerResult("");
        setIsQuestion(true);
        setOthersAnswer("");
        setTimeLimit(tl);
        start(tl);

        setIsTimeUp(false);
      } else if (command == "/ans_lock") {
        setIsAnswerLock(true);
      } else if (command == "/ans_unlock") {
        setIsAnswerLock(false);
      } else if (command == "/timeup") {
        setIsTimeUp(true);
        setCurrentQuestion("Time Up!");
      } else if (command == "/question_answer") {
        const answer = split[1];
        console.log("答え" + answer);

        setCurrentQuestionAnswer(answer);
      } else if (command == "/explanatory") {
        const explanatory = split[1];

        setIsQuestion(false);
        setCurrentQuestionExplanatory(explanatory);
      } else if (command == "/others_correct_answer") {
        const id = Number.parseInt(split[1]);
        const answer = split[2];

        setAnswerResult("○");
        setOthersAnswer(answer);
        console.log(answer);
      } else if (command == "/others_incorrect_answer") {
        const id = Number.parseInt(split[1]);
        const answer = split[2];

        setAnswerResult("✕");
        setOthersAnswer(answer);
        console.log(answer);
      } else if (command == "/users") {
        const userData: string = data.data;
        const userJson = userData.split(" ", 2)[1];

        console.log("json" + userJson);
        const json = JSON.parse(userJson) as User[];
        console.log("in eff" + json);
        setUserData(json);
      } else if (command == "/join_ok") {
        console.log("OKKKKKK");
        setIsWelcome(false);
      } else if (command == "/join_err") {
        setIsWelcome(true);
      } else if (command == "/name_ok") {
      } else if (command == "/name_err") {
      } else if (command == "/quiz_started") {
      } else if (command == "/ans_ok") {
        setIsAnswerRight(true);
      } else if (command == "/ans_err") {
        setIsAnswerRight(false);
      } else if (command == "/correct") {
        setAnswerResult("正解！");
      } else if (command == "/incorrect") {
        setAnswerResult("残念...");
      } else if (command == "/result") {
        setIsEnd(true);
      }
    };

    webSocket.onclose = (e) => {
      console.log("WebSocket Closed");
    };

    webSocket.onerror = (e) => {
      console.log("ws Err");
    };
  }, []);

  const sendJoin = (roomName: string) => {
    webSocket.send("/join " + roomName);
  };
  const sendName = (name: string) => {
    webSocket.send("/name " + name);
  };
  const setIsWelcomeFalse = () => {
    setIsWelcome(false);
  };
  const sendStart = () => {
    webSocket.send("/start");
  };
  const sendAnsReq = () => {
    if (!isAnswerLock) {
      webSocket.send("/ans_req");
    }
  };
  const sendAnswer = (answer: string) => {
    if (isAnswerRight) {
      webSocket.send("/answer " + answer);
      setIsAnswerRight(false);
      setOthersAnswer(answer);
    }
  };

  return (
    <div>
      {isWelcome ? (
        <Welcome sendJoin={sendJoin} sendName={sendName} />
      ) : !isEnd ? (
        <Quiz
          sendStart={sendStart}
          userData={userData}
          isStarted={isStarted}
          currentQuestion={currentQuestion}
          currentQuestionAnswer={currentQuestionAnswer}
          answerResult={answerResult}
          othersAnswer={othersAnswer}
          sendAnsReq={sendAnsReq}
          isAnswerRight={isAnswerRight}
          sendAnswer={sendAnswer}
          isQuestion={isQuestion}
          currentQuestionExplanatory={currentQuestionExplanatory}
          timeLeftSec={timeLeftMs / 1000}
        />
      ) : (
        <Result userData={userData} />
      )}
    </div>
  );
};

export default App;

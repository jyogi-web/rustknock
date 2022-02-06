import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import Welcome from "./Welcome";
import Quiz from "./Quiz";
import { stringify } from "querystring";
import { type } from "os";
// import socketIOClient from "socket.io-client";

const URL = "wss://rustknock-server.azurewebsites.net/ws/";
// const URL = "ws://localhost:3000/ws/";
const webSocket = new WebSocket(URL);

type User = {
  id: number;
  name: string;
  score: number;
};

type Users = {
  userdata: User[];
};

interface Props {}

const App: React.FC<Props> = (props) => {
  const [isWelcome, setIsWelcome] = useState(true);
  const [currentQuestion, setCurrentQuestion] = useState("");
  const [isAnswerLock, setIsAnswerLock] = useState(false);
  // TODO timeup
  const [currentQuestionAnswer, setCurrentQuestionAnswer] = useState("");
  const [currentQuestionExplanatory, setCurrentQuestionExplanatory] =
    useState("");
  // TODO others_correct_answer
  // TODO others_incorrect_answer

  useEffect(() => {
    webSocket.onopen = () => {
      console.log("WebSocket Connected");
    };
    webSocket.onmessage = (data) => {
      console.log(data.data);
      const msg: string = data.data;

      if (data.data.startsWith("/quiz_started")) {
      } else if (data.data.startsWith("/question")) {
        const split: string = data.data.split(" ");
        const timeLimitMs = Number.parseInt(split[1]);
        const question = split[2];
      } else if (data.data.startsWith("/ans_lock")) {
      } else if (data.data.startsWith("/ans_unlock")) {
      } else if (data.data.startsWith("/timeup")) {
      } else if (data.data.startsWith("/question_answer")) {
      } else if (data.data.startsWith("/explanatory")) {
      } else if (data.data.startsWith("/others_correct_answer")) {
        const split: string = data.data.split(" ");
        const id = Number.parseInt(split[1]);
        const answer = split[2];
      } else if (data.data.startsWith("/others_incorrect_answer")) {
        const split: string = data.data.split(" ");
        const id = Number.parseInt(split[1]);
        const answer = split[2];
      } else if (data.data.startsWith("/users")) {
        const userData: string = data.data;
        const userJson = userData.split(" ", 2)[1];

        console.log(JSON.parse(userJson) as Users);
      } else if (data.data.startsWith == "/join_ok") {
        console.log("OKKKKKK");
      } else if (data.data.startsWith("/join_err")) {
      } else if (data.data.startsWith("/name_ok")) {
      } else if (data.data.startsWith("/name_err")) {
      } else if (data.data.startsWith("/quiz_started")) {
      } else if (data.data.startsWith("/ans_ok")) {
      } else if (data.data.startsWith("/ans_err")) {
      } else if (data.data.startsWith("/correct")) {
      } else if (data.data.startsWith("/incorrect")) {
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
    webSocket.send("/ans_req");
  };
  const sendAnswer = (answer: string) => {
    webSocket.send("/answer " + answer);
  };

  return (
    <div>
      {isWelcome ? (
        <Welcome
          sendJoin={sendJoin}
          sendName={sendName}
          setIsWelcomeFalse={setIsWelcomeFalse}
        />
      ) : (
        <Quiz />
      )}
    </div>
  );
};

export default App;

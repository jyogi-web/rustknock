import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import Welcome from "./Welcome";
import Quiz from "./Quiz";
import socketIOClient from "socket.io-client";

const URL = "ws://localhost:3000/ws/";
const webSocket = new WebSocket(URL);

interface Props {}

const App: React.FC<Props> = (props) => {
  const [isWelcome, setIsWelcome] = useState(true);

  useEffect(() => {
    webSocket.onopen = () => {
      console.log("WebSocket Connected");
    };
    webSocket.onmessage = (data) => {
      console.log(data.data);

      if (data.data == "/quiz_started") {
      } else if (data.data == "/question") {
      } else if (data.data == "/ans_lock") {
      } else if (data.data == "/ans_unlock") {
      } else if (data.data == "/timeup") {
      } else if (data.data == "/question_answer") {
      } else if (data.data == "/others_correct_answer") {
      } else if (data.data == "/others_incorrect_answer") {
      } else if (data.data == "/users") {
      } else if (data.data == "/join_ok") {
      } else if (data.data == "/join_err") {
      } else if (data.data == "/name_ok") {
      } else if (data.data == "/name_err") {
      } else if (data.data == "/quiz_started") {
      } else if (data.data == "/ans_ok") {
      } else if (data.data == "/ans_err") {
      } else if (data.data == "/correct") {
      } else if (data.data == "/incorrect") {
      }
    };

    webSocket.onclose = (e) => {
      console.log("WebSocket Closed");
    };
  }, []);

  const sendJoin = (roomName: string) => {
    webSocket.send("/join " + roomName);
  };
  const sendName = (name: string) => {
    webSocket.send("/name " + name);
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

  // ためしに
  const onclick = () => {
    setIsWelcome(false);
  };
  return (
    <div>
      {isWelcome ? <Welcome /> : <Quiz />}
      <button onClick={onclick}>test</button>
    </div>
  );
};

export default App;

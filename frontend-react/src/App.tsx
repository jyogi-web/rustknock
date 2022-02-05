import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import Welcome from "./Welcome";
import Quiz from "./Quiz";

interface Props {}

const App: React.FC<Props> = (props) => {
  const [isWelcome, setIsWelcome] = useState(true);

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

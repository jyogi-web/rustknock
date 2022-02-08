import PropTypes from "prop-types";
import { useEffect, useState } from "react";
import { User, Users } from "./App";
import ScoreBoardItem from "./ScoreBoardItem";
interface Props {
  sendStart: () => void;
  userData: User[];
  isStarted: boolean;
  currentQuestion: string;
  currentQuestionAnswer: string;
  answerResult: boolean;
  othersAnswer: string;
  sendAnsReq: () => void;
  isAnswerRight: boolean;
  sendAnswer: (ans: string) => void;
}

interface ScoreData {
  username: string;
  score: number;
}

const Quiz: React.FC<Props> = (props) => {
  const [scoreData, setScoreData] = useState<User[]>([]);

  useEffect(() => {
    const sampleData: User[] = [
      {
        id: 0,
        name: "nissie",
        score: 20,
      },
      {
        id: 0,
        name: "nissie",
        score: 20,
      },
      {
        id: 0,
        name: "nissie",
        score: 20,
      },
    ];
    setScoreData(sampleData);
  }, []);

  let myAnswer = "";
  const inputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    myAnswer = e.target.value;
  };
  const sendMyAnswer = () => {
    props.sendAnswer(myAnswer);
  };
  console.log(props.userData);

  return (
    <div className="quiz">
      {/* メインコンテンツ */}
      <div className="main grid grid-cols-3 pt-20 h-full">
        {/* スコアボード */}
        <div className="scoreboard grid grid-rows">
          <div className="scoreboard-title flex justify-center items-center border-4 border-sky-200 p-1 mx-20 rounded-t-lg">
            スコアボード
          </div>
          <div className="scoreboard-content">
            <div className="scoreboard-list">
              {props.userData &&
                props.userData.map((item: User) => (
                  <ScoreBoardItem name={item.name} score={item.score} />
                ))}
            </div>
          </div>
          <div className="scoreboard-title flex justify-center items-center border-4 border-sky-200 p-1 mx-20 rounded-b-lg"></div>
        </div>
        {/* モニター */}
        <div className="monitor border-2 border-black flex justify-center items-center text-center rounded-lg">
          <div className="monitor-start">
            <div className="monitor-text">
              {props.isStarted
                ? "問題 " + props.currentQuestion
                : "メンバーは揃いましたか？"}
            </div>
            {props.isStarted ? (
              <div />
            ) : (
              <button
                onClick={props.sendStart}
                className="start-button border-2 border-black rounded mt-10 px-10 py-2"
              >
                開始する
              </button>
            )}
          </div>
        </div>
        {/* 進行状況 */}
        <div className="proggress">
          {/* 解答 */}
          <div className="user-answer grid grid-rows-1 flex justify-center items-center text-center border-4 border-sky-200 p-1 mx-20 rounded-t-lg">
            <div className="user-answer-key">解答</div>
            <div className="user-answer-value">{props.othersAnswer}</div>
          </div>
          {/* 答え */}
          <div className="currect-answer user-answer grid grid-rows-1 flex justify-center items-center text-center border-4 border-sky-200 p-1 my-1 mx-20">
            <div className="currect-answer-key">答え</div>
            <div className="currect-answer-value">
              {props.currentQuestionAnswer}
            </div>
          </div>
          {/* 結果 */}
          <div className="result user-answer grid grid-rows-1 flex justify-center items-center text-center border-4 border-sky-200 p-1 mx-20 rounded-b-lg">
            <div className="result-key">結果</div>
            <div className="result-value">{props.answerResult ? "○" : "✕"}</div>
          </div>
        </div>
      </div>
      {/* 早押しボタン */}
      <div className="fast-press flex items-center justify-center">
        {/* ボタン */}
        <button
          onClick={props.sendAnsReq}
          className="fast-press-button border-2 border-rose-700 py-5 px-10 rounded-lg bg-red-600 text-white font-bold text-lg mt-20"
        >
          {props.isAnswerRight ? "解答を送信してください" : "早押しボタン"}
        </button>
      </div>
      {/* 解答フォーム */}
      <form className="answer-form flex items-center justify-center mt-20">
        {/* 入力フォーム */}
        <label htmlFor="answer" className="sr-only">
          解答
        </label>
        <input
          onChange={inputChange}
          type="text"
          name="answer"
          id="answer"
          className=""
        />
        {/* 送信ボタン */}
        <button
          onClick={sendMyAnswer}
          className="answer-button rounded-lg border-2 border-sky-700 p-2 bg-sky-500 text-white"
          type="button"
        >
          送信
        </button>
      </form>
    </div>
  );
};

export default Quiz;

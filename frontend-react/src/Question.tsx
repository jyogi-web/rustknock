interface Props {
  isQuestion: boolean;
  currentQuestion: string;
  currentQuestionExplanatory: string;
}

const Question: React.FC<Props> = (props) => {
  const explanatory = () => {
    return (
      <div>
        <div className="rounded-lg border-2 border-red-400 p-2 text-red-500 text-xl">
          解説
        </div>
        <div> {props.currentQuestionExplanatory}</div>
      </div>
    );
  };

  return (
    <div>
      <div className="border-2 border-black p-2 text-black">問題</div>
      <div>{props.currentQuestion}</div>
      {!props.isQuestion ? explanatory() : <div />}
    </div>
  );
};

export default Question;

interface Props {
  name: string;
  score: number;
}

const ScoreBoardItem: React.FC<Props> = (props) => {
  return (
    <div className="scoreboard-item grid grid-cols-2 border-4 border-sky-200 rounded mx-20 my-1">
      <div className="scoreboard-username p-1 flex justify-center items-center">
        {props.name}
      </div>
      <div className="scoreboard-score p-1 flex justify-center items-center">
        {props.score}pt
      </div>
    </div>
  );
};

export default ScoreBoardItem;

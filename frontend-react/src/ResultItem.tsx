interface Props {
  order: number;
  name: string;
  score: number;
}

const ResultItem: React.FC<Props> = (props) => {
  return (
    <div className="result-item flex border-4 border-sky-200 rounded mx-40 my-1 py-2">
      <div className="result-username p-1 flex-auto justify-center items-center justify-self-end text-center text-3xl">
        {props.order}
      </div>
      <div className="result-username p-1 flex-auto justify-center items-center text-center text-2xl">
        {props.name}
      </div>
      <div className="result-score p-1 flex-auto justify-center items-center text-center text-2xl">
        {props.score}pt
      </div>
    </div>
  );
};

export default ResultItem;

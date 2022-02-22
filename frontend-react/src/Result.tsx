import { User } from "./App";
import ResultItem from "./ResultItem";
import ScoreBoardItem from "./ScoreBoardItem";

interface Props {
  userData: User[];
}

const Result: React.FC<Props> = (props) => {
  const userData = props.userData.sort((a, b) => {
    if (a.score > b.score) {
      return -1;
    } else if (a.score < b.score) {
      return 1;
    } else {
      return 0;
    }
  });

  return (
    <div className="flex flex-col">
      <div className="text-6xl text-center border-4 border-green-400 rounded mx-20 my-4 py-4">
        Result
      </div>
      {userData &&
        userData.map((item: User, index: number) => (
          <ResultItem order={index + 1} name={item.name} score={item.score} />
        ))}
    </div>
  );
};

export default Result;

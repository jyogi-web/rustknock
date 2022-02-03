use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Quiz {
//     id: isize,
//     question: String,
//     answer: String,
//     explanatory: String,
// }

// type Quizzes = Vec<Quiz>;

// pub(crate) fn read_file() -> Quizzes {
//     let file_name = "quizzes.json";
//     let file = File::open(file_name).unwrap();
//     let reader = BufReader::new(file);
//     let quizzes: Quizzes = serde_json::from_reader(reader).unwrap();
//     quizzes
// }

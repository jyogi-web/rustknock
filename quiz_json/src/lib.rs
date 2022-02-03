#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use getset::Getters;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct Quiz {
    #[getset(get = "pub")]
    id: isize,
    #[getset(get = "pub")]
    question: String,
    #[getset(get = "pub")]
    answer: String,
    #[getset(get = "pub")]
    explanatory: String,
}

pub type Quizzes = Vec<Quiz>;

pub fn read_file<P>(path: P) -> Quizzes
where
    P: AsRef<Path>,
{
    let file_name = path.as_ref();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let quizzes: Quizzes = serde_json::from_reader(reader).unwrap();
    quizzes
}

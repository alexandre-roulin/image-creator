#[derive(Debug)]
pub enum MyError {
    Csv(csv::Error),
}

impl From<csv::Error> for MyError {
    fn from(e: csv::Error) -> Self {
        MyError::Csv(e)
    }
}

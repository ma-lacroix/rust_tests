use std::error::Error;
use csv::Reader;

fn main() {
    let path: String = "data.csv".to_string();
    let mut data: file_reader::reading::FileToRead = file_reader::reading::FileToRead::new(path);
    data.create_vectors();
    data.read_vectors();
}

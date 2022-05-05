use std::error::Error;
use file_reader::{dataframes, SHOW_ROWS};
use file_reader::reading::{FileToRead, read_csv};


fn main() {
    // TODO: find a way to make this automatically retrievable from the internet

    // Pull csv data
    let path: String = "covid19.csv".to_string();
    let data: FileToRead = read_csv(path);

    // create string vectors "DataFrame"
    let mut strings = dataframes::StringDf::new(&data.vector_registry_all, vec![0, 1]);
    dataframes::head(&mut strings, SHOW_ROWS);
    dataframes::min(&mut strings, 0);

    // create i32 vectors "DataFrame"
    let mut numbers = dataframes::IntDf::new(&data.vector_registry_all, vec![2, 3, 4, 5]);
    dataframes::head(&mut numbers, SHOW_ROWS);
    dataframes::min(&mut numbers, 0);
    dataframes::max(&mut numbers, 0);
    dataframes::mean(&mut numbers, 0);
}

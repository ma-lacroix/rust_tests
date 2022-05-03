use std::error::Error;
use csv::Reader;
use polars::export::arrow::compute::arithmetics::basic::mul;
use file_reader::{dataframes, SHOW_ROWS, vector_data};
use file_reader::dataframes::{print_vectors, vector_avg, vector_max, vector_min};

fn main() {
    // TODO: find a way to make this automatically retrievable from the internet

    // Pull csv data
    let path: String = "covid19.csv".to_string();
    let mut data: file_reader::reading::FileToRead = file_reader::reading::FileToRead::new(path);
    data.fill_vectors();

    // create string vectors "DataFrame"
    let mut strings = dataframes::StringDf::new(&data.vector_registry_all, vec![0, 1]);
    print_vectors(&mut strings, SHOW_ROWS);
    vector_min(&mut strings, 0);

    // create i32 vectors "DataFrame"
    let mut numbers = dataframes::IntDf::new(&data.vector_registry_all, vec![2, 3, 4, 5]);
    print_vectors(&mut numbers, SHOW_ROWS);
    vector_min(&mut numbers, 0);
    vector_max(&mut numbers, 0);
    vector_avg(&mut numbers, 0);
}

use std::error::Error;
use csv::Reader;
use file_reader::SHOW_ROWS;
use file_reader::vector_data::*;

fn main() {
    // TODO: find a way to make this automatically retrievable from the internet
    let path: String = "covid19.csv".to_string();
    let mut data: file_reader::reading::FileToRead = file_reader::reading::FileToRead::new(path);
    data.fill_vectors();
    data.convert_to_string(vec![0,1]);
    data.convert_to_int(vec![2,3,4,5]);
    data.read_vectors(SHOW_ROWS);
    let max_value = get_max_int_value(&data.vector_registry_int[0]);
    let min_value = get_min_int_value(&data.vector_registry_int[0]);
    let avg_value = get_average_value(&data.vector_registry_int[0]);
}

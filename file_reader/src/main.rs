use file_reader::SHOW_ROWS;
use file_reader::vector_data::*;

fn main() {
    // TODO: find a way to make this automatically retrievable from the internet
    let path: String = "covid19.csv".to_string();
    let mut data: file_reader::reading::FileToRead = file_reader::reading::FileToRead::new(path);
    data.create_vector_registry();
    data.fill_vectors();
    data.read_vectors(SHOW_ROWS);
    let some_vec: Vec<i32> = convert_to_int(&data.vector_registry[3]);
    let max_value = get_max_int_value(&some_vec);
    let min_value = get_min_int_value(&some_vec);
    let avg_value = get_average_value(&some_vec);
}

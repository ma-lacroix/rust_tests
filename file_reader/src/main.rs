use file_reader::SHOW_ROWS;

fn main() {
    let path: String = "covid19.csv".to_string();
    let mut data: file_reader::reading::FileToRead = file_reader::reading::FileToRead::new(path);
    data.create_vector_registry();
    data.fill_vectors();
    data.read_vectors(SHOW_ROWS);
}

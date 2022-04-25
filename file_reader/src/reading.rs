use crate::NUM_COLS;
use csv::Reader;
use std::error::Error;
use std::fs::File;

#[allow(unused_results)]
pub struct FileToRead {
    pub filepath: String,
    pub vector_registry: Vec<Vec<String>>,
}

impl FileToRead {
    pub fn new(filename: String) -> Self {
        Self {
            filepath: filename.to_string(),
            vector_registry: vec![vec![]; NUM_COLS],
        }
    }
    pub fn create_vector_registry(&mut self) {
        for _n in 0..NUM_COLS {
            self.vector_registry.push(vec![]);
        }
    }
    pub fn fill_vectors(&mut self) -> Result<(), Box<dyn Error>> {

        let mut reader: Reader<File> = csv::Reader::from_path(&self.filepath)?;

        for item in reader.records() {
            let record = item?;
            for n in 0..NUM_COLS {
                self.vector_registry[n].push((&record[n]).parse().unwrap());
            }
        }
        Ok(())
    }
    pub fn read_vectors(&mut self, max_rows: usize) {
        //     print a few rows
        for i in 0..max_rows {
            for j in 0..NUM_COLS {
                print!("{}, ", self.vector_registry[j][i]);
            }
            println!();
        }
    }
}

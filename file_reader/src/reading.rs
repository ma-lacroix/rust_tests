use std::error::Error;
use std::fs::File;
use csv::{Reader, StringRecord};

pub struct FileToRead {
    pub filepath: String,
    pub vec1: Vec<String>,
    pub vec2: Vec<String>,
}

impl FileToRead {
    pub fn new(filename: String) -> Self {
        Self {
            filepath: filename.to_string(),
            vec1: vec![],
            vec2: vec![]
        }
    }
    pub fn create_vectors(&mut self) -> Result<(), Box<dyn Error>>  {
        let mut reader: Reader<File> = csv::Reader::from_path(&self.filepath)?;

        for item  in reader.records() {
            let record = item?;
            self.vec1.push((&record[0]).parse().unwrap());
            self.vec2.push((&record[1]).parse().unwrap());
        }
        Ok(())
    }
    pub fn read_vectors(&mut self) {
        for item in self.vec1.iter().zip(self.vec2.iter()) {
            println!("{}, {}", item.0, item.1);
        }
    }
}
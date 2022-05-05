use std::error::Error;

#[allow(unused_results)]
pub struct FileToRead {
    pub filepath: String,
    pub vector_registry_all: Vec<Vec<String>>,
    pub vector_registry_string: Vec<Vec<String>>,
    pub vector_registry_int: Vec<Vec<i32>>,
    pub num_cols: usize,
}

impl FileToRead {
    pub fn new(filename: String) -> Self {
        Self {
            filepath: filename.to_string(),
            vector_registry_all: vec![vec![]; 0],
            vector_registry_string: vec![vec![]; 0],
            vector_registry_int: vec![vec![]; 0],
            num_cols: 0,
        }
    }

    pub fn create_vector_registry_all(&mut self) {
        for _n in 0..self.num_cols {
            self.vector_registry_all.push(vec![]);
        }
    }

    pub fn fill_vectors(&mut self) -> Result<(), Box<dyn Error>> {

        // read the file
        let mut reader = csv::Reader::from_path(&self.filepath)?;
        let mut headers = reader.headers();

        // get number of columns
        self.num_cols = headers.unwrap().len();
        self.create_vector_registry_all();

        // fill the data into a pseudo-DF - all strings at this point
        for item in reader.records() {
            let record = item?;
            for n in 0..self.num_cols {
                self.vector_registry_all[n].push((&record[n]).parse().unwrap());
            }
        }
        Ok(())
    }
}

pub fn read_csv(path: String) -> FileToRead {
    let mut data: FileToRead = FileToRead::new(path);
    data.fill_vectors();
    data
}

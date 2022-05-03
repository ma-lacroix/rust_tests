use csv::{Reader, StringRecord};
use std::error::Error;
use std::fs::File;

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

    pub fn convert_to_int(&mut self, indexes: Vec<i32>) {
        for index in indexes {
            let u_index: usize = index as usize;
            println!("Handling: {}", index);
            let numbers_vec: Vec<i32> = self.vector_registry_all[u_index].iter()
                .map(|x| x.parse().unwrap()).collect();
            self.vector_registry_int.push(numbers_vec);
        }
    }

    pub fn convert_to_string(&mut self, indexes: Vec<i32>) {
        for index in indexes {
            let u_index: usize = index as usize;
            let strings_vec: Vec<String> = self.vector_registry_all[u_index].clone();
            self.vector_registry_string.push(strings_vec);
        }
    }

    pub fn read_vectors(&mut self, max_rows: usize) {
        // Prints a few rows
        for i in 0..max_rows {
            for j in 0..self.vector_registry_string.len() {
                print!("{}, ", self.vector_registry_string[j][i]);
            }
            println!();
        }
        println!("  ****  ");
        for i in 0..max_rows {
            for j in 0..self.vector_registry_int.len() {
                print!("{}, ", self.vector_registry_int[j][i]);
            }
            println!();
        }
        println!("  ****  ");
    }
}

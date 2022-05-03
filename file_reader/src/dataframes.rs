// Structs that mimic dataframes by data type

pub struct StringDf {
    pub vector_registry: Vec<Vec<String>>,
}

pub struct IntDf {
    pub vector_registry: Vec<Vec<i32>>,
}

impl IntDf {
    pub fn new(vector: &Vec<Vec<String>>, indexes: Vec<i32>) -> Self {

        let mut temp_vec: Vec<Vec<i32>> = Vec::new();
        for index in indexes {
            let u_index: usize = index as usize;
            let numbers_vec: Vec<i32> = vector[u_index].iter()
                .map(|x| x.parse().unwrap()).collect();
            temp_vec.push(numbers_vec);
        }

        Self {
            vector_registry: temp_vec,
        }
    }
}

impl StringDf {
    pub fn new(vector: &Vec<Vec<String>>, indexes: Vec<i32>) -> Self {

        let mut temp_vec: Vec<Vec<String>> = Vec::new();
        for index in indexes {
            let u_index: usize = index as usize;
            let strings_vec: Vec<String> = vector[u_index].clone();
            temp_vec.push(strings_vec);
        }

        Self {
            vector_registry: temp_vec,
        }
    }
}

pub trait VectorUtils {
    fn print_elements(self: &mut Self, max_rows: usize);
    fn get_max_element(self: &mut Self, index: usize) -> i32;
    fn get_min_element(self: &mut Self, index: usize) -> i32;
    fn get_average_value(self: &mut Self, index: usize) -> i32;
}

impl VectorUtils for StringDf {
    fn print_elements(&mut self, max_rows: usize) {
        for i in 0..max_rows {
            for j in 0..self.vector_registry.len() {
                print!("{}, ", self.vector_registry[j][i]);
            }
            println!();
        }
        println!("  ****  ");
    }
    fn get_max_element(self: &mut Self, index: usize) -> i32 {
        println!("\nError, this is a String type!\n");
        let value = 0;
        value
    }
    fn get_min_element(self: &mut Self, index: usize) -> i32 {
        println!("\nError, this is a String type!\n");
        let value = 0;
        value
    }
    fn get_average_value(self: &mut Self, index: usize) -> i32 {
        println!("\nError, this is a String type!\n");
        let value = 0;
        value
    }
}

impl VectorUtils for IntDf {
    // same as string_df struct for now
    fn print_elements(&mut self, max_rows: usize) {
        for i in 0..max_rows {
            for j in 0..self.vector_registry.len() {
                print!("{}, ", self.vector_registry[j][i]);
            }
            println!();
        }
        println!("  ****  ");
    }
    fn get_max_element(self: &mut Self, index: usize) -> i32 {
        let max_value: i32 = *self.vector_registry[index].iter().max().unwrap();
        println!("\nMax value: {}", max_value);
        max_value
    }
    fn get_min_element(self: &mut Self, index: usize) -> i32 {
        let min_value: i32 = *self.vector_registry[index].iter().min().unwrap();
        println!("\nMin value: {}", min_value);
        min_value
    }
    fn get_average_value(self: &mut Self, index: usize) -> i32 {
        let vec_length: i32 = self.vector_registry[index].len() as i32;
        let total: i32 = self.vector_registry[index].iter().sum();
        println!("\nAvg value: {}", total/vec_length);
        total/vec_length
    }

}

pub fn print_vectors<T: VectorUtils>(item: &mut T, max_rows: usize) {
    item.print_elements(max_rows);
}

pub fn vector_min<T: VectorUtils>(item: &mut T, index: usize) -> i32 {
    let value: i32 = item.get_min_element(index);
    value
}

pub fn vector_max<T: VectorUtils>(item: &mut T, index: usize) -> i32 {
    let value: i32 = item.get_max_element(index);
    value
}

pub fn vector_avg<T: VectorUtils>(item: &mut T, index: usize) -> i32 {
    let value: i32 = item.get_average_value(index);
    value
}
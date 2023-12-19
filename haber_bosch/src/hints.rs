#![allow(unused)]
use std::fmt::Display; // also this gives a warning we need it after uncommenting some lines

// hint: this uses all the hints given for the task in a central place:
pub fn hint_main() {
    
    // create two data structures
    let data1 = DataStructure::new();
    let mut data2 = DataStructure::new();

    // declare a variable and call a function:
    // @todo comment my_func_ref and instead use my_fun_move, investigate the error messages
    let our_function_reval = my_func_ref(&data1);
    // let our_function_reval = my_func_move(data1);

    // shadow a variable
    let number = data1.pythagoras_c();

    data2.double_me();

    println!("We can put formatted text here. reval is {}, C is {}", our_function_reval, number);
    println!("We can dump data that derives/impl the Debug trait:\n{:?}", data1);
    println!("{:?}", data2);

    // @todo uncomment, read error, then uncomment trait at the end of file to see pretty print.
    //println!("Pretty print: {}", data2);
}

// hint: How to implement a function
// The function name my_func_ref has one argument 'data' of type 'reference to DataStructure' it returns a f64
fn my_func_ref(data: &DataStructure) -> f64 {
    // we just add both fields of the struct
    data.var_name + data.another_var
}
fn my_func_move(data: DataStructure) -> f64 {
    data.var_name + data.another_var
}

// hint: how to use a data structure
// this is called deriving a trait. This adds functionality to the type below.
#[derive(Debug)] 
// it follows the type definition which is public such that we can use it in main.
struct DataStructure {
    pub var_name: f64,      // f64 is the type and indicates a 64 bit floating point number
    pub another_var: f64,
}


// the following is an implementation block, here we implement methods for DataStructure
impl DataStructure {

    // this is like a constructor it is called associated function as it is associated with the type.
    // It returns an object of 'Self' which is a DataStructure
    fn new() -> Self {
        DataStructure { 
            var_name: 2.0, 
            another_var: 3.0 
        }
    }

    // this is a method, &self means we have a 'readonly' reference to itself
    fn pythagoras_c(&self) -> f64 {
        // omitting the semicolon in the line below is returning the expression
        (self.var_name.powi(2) + self.another_var.powi(2)).sqrt()
    }

    /// this is another method, this time self must be mutable which allows the method to change the itself
    fn double_me(&mut self) {
        self.var_name = self.var_name * 2.;
        self.another_var = self.another_var * 2.;
    }
}

// @todo Uncomment
/*
// here we'll implement the Display trait for DataStructure to enable a pretty print
impl Display for DataStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Data Structure: {}^2 + {}^2 = {}^2", 
            self.var_name, self.another_var, self.pythagoras_c()).as_str())
    }
}
*/
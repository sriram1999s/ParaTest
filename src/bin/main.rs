use regex::Regex;
use std::env;
use std::fs;

use para_test::TestFunctions;

fn main() {

    // initializing store
    let mut store = TestFunctions::new();
    // reading the command line arguments
    let args: Vec<String> = env::args().collect();
    let test_filename = &args[1];
    let test_file_contents = fs::read_to_string(test_filename).expect("Something went wrong reading the file");

    // finding all test functions and storing
    let test_fn_pattern = Regex::new(r"/\* test \*/[\s]*.*?\{").unwrap();
    for mat in test_fn_pattern.find_iter(test_file_contents.as_str()) {
        let temp =  &test_file_contents[mat.start()..mat.end()];
        let func = Regex::new(r"void\b\s*(.*?)\b?\(\)").unwrap().captures(temp).unwrap();
        let func = func.get(1).unwrap().as_str();
        store.add(func);
    }
    store.display();
}

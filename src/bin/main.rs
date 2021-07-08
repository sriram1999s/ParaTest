use regex::Regex;
use std::env;
use std::fs;

use para_test::TestFunctions;

const DEBUG: bool = false;

fn main() {
    // initializing store
    let mut store = TestFunctions::new();

    // reading the command line arguments
    let args: Vec<String> = env::args().collect();
    let test_filename = &args[1];
    let interface_path = &args[2];
    let impl_filename = &args[3];
    let test_file_contents = fs::read_to_string(test_filename)
        .expect("Something went wrong reading the test specification file");
    let impl_file_contents = fs::read_to_string(impl_filename)
        .expect("Something went wrong reading the implementation file");

    if DEBUG {
        println!("interface path:\n{}\n", interface_path);
        println!("test spec file :\n{}\n", test_file_contents);
        println!("impl file :\n{}", impl_file_contents);
    }

    // finding all test functions and storing
    let test_fn_pattern = Regex::new(r"/\* test \*/[\s]*.*?\s*?\{").unwrap();
    for mat in test_fn_pattern.find_iter(test_file_contents.as_str()) {
        let temp = &test_file_contents[mat.start()..mat.end()];
        let func = Regex::new(r"void\b\s*(.*?)\b?\(\)")
            .unwrap()
            .captures(temp)
            .unwrap();
        let func = func.get(1).unwrap().as_str();
        store.add(func);
    }

    if DEBUG {
        store.display();
    }

    // modifying tests spec file
    para_test::modify_test_spec_file(test_filename, interface_path);

    // creating a header file
    store.create_header();

    // running test
    store.run_tests(test_filename, impl_file_contents.as_str());
}

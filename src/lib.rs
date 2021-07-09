// use rayon::prelude::*;

use regex::Regex;

use std::fs;
use std::process::Command;
use std::env;

pub struct TestFunctions {
    list: Vec<String>,
}

impl TestFunctions {
    pub fn new() -> TestFunctions {
        TestFunctions { list: Vec::new() }
    }
    pub fn add(&mut self, function: &str) {
        self.list.push(String::from(function));
    }

    pub fn display(&self) {
        for func in &self.list {
            println!("{}\n", func);
        }
    }

    fn get_path() -> Option<String> {
        match env::var("PARATEST_PATH") {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
    /* creating header file */
    pub fn create_header(&self) {
        let mut declarations = String::from("");
        for func in &self.list {
            declarations.push_str(format!("void {}();\n", func).as_str());
        }
        let path = TestFunctions::get_path().unwrap();
        fs::write(path + "/cpp_files/tests.hpp", declarations.as_str())
            .expect("Something went wrong when creating tests.hpp");
    }

    // run all tests /* currently sequential */
    pub fn run_tests(&self, tests_path: &str, impl_file: &str) {
        self.list.iter().for_each(|func| {

            let path = TestFunctions::get_path().unwrap();
            let mut main_file = String::from(format!("#include \"{}/cpp_files/tests.hpp\"\n", path).as_str());
            main_file.push_str(impl_file);
            main_file.push_str(format!("int main() {{\n\t{}();\n}}", func).as_str());

            let path = TestFunctions::get_path().unwrap();

            let main_path = String::from(path) + "/cpp_files/main.cpp";
            let main_path = main_path.as_str();

            fs::write(main_path, main_file.as_str())
                .expect("Something went wrong when creating main.cpp");

            let output = Command::new("g++")
                .arg(main_path)
                .arg(tests_path)
                .output()
                .expect("failed to compile");

            assert!(output.status.success());

            let exec = Command::new("./a.out").output().expect("Error when running exec");

            match exec.status.success() {
                true => println!("{} Passed : was completed successfully!\n", func ),
                false => println!("{} Failed : {}", func, String::from_utf8_lossy(&exec.stderr)),
            }

            let clean = Command::new("rm")
                .arg("a.out")
                .output()
                .expect("failed to remove binary");

            assert!(clean.status.success());

        })
    }
}

// modifying tests specification file to add interface header if not present
pub fn modify_test_spec_file(tests_file_path: &str, interface: &str) {
    let mut header = format!("#include \"{}\"\n", interface);
    let pat = Regex::new(header.as_str()).unwrap();

    let tests_file = fs::read_to_string(tests_file_path).expect("Something went wrong reading the test specification file");
    if ! pat.is_match(tests_file.as_str()) {
        header.push_str(tests_file.as_str());
        fs::write(tests_file_path, header).expect("Something went wrong when modifying tests spec");
    }
}

// use foreach::*;
use rayon::prelude::*;

use std::fs;
use std::process::Command;

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

    /* creating header file */
    pub fn create_header(&self) {
        let mut declarations = String::from("");
        for func in &self.list {
            declarations.push_str(format!("void {}();\n", func).as_str());
        }
        fs::write("../cpp_files/tests.hpp", declarations.as_str())
            .expect("Something went wrong when creating tests.hpp");
    }

    // run all tests // without threading
    pub fn run_tests(&self, impl_file: &str) {
        self.list.iter().for_each(|func| {
            let mut main_file = String::from("#include \"tests.hpp\"\n");
            main_file.push_str(impl_file);
            main_file.push_str(format!("int main() {{\n\t{}();\n}}", func).as_str());

            fs::write("../cpp_files/main.cpp", main_file.as_str())
                .expect("Something went wrong when creating main.cpp");

            let output = Command::new("g++")
                .arg("../cpp_files/main.cpp")
                .arg("../cpp_files/tests.cpp")
                .output()
                .expect("failed to execute process");

            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            assert!(output.status.success());
        })
    }
}
